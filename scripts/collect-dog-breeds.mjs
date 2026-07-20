import { mkdir, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const directoryPath = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const outputPath = resolve(directoryPath, "data", "breeds", "dogs.json");

const AKC_LIST_URL = "https://www.akc.org/dog-breeds/";
const AKC_BREED_URL = "https://www.akc.org/dog-breeds/";

const USER_AGENT =
  "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

const REQUEST_HEADERS = {
  "User-Agent": USER_AGENT,
  Accept:
    "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
  "Accept-Language": "en-US,en;q=0.9",
  "Cache-Control": "no-cache",
  Pragma: "no-cache",
  "sec-ch-ua-platform": '"macOS"',
  "sec-fetch-dest": "document",
  "sec-fetch-mode": "navigate",
  "sec-fetch-site": "none",
  "upgrade-insecure-requests": "1",
};

const CONCURRENCY_LIMIT = 5;
const MAX_RETRIES = 3;
const BASE_DELAY_MILLIS = 300;
const EXTRA_DELAY_MILLIS = 200;

// AKC coat_type uses Wiry, Hairless, Smooth, Rough, Corded, Double, Curly, Wavy, Silky.
// We map to the Rust enum coat_type: short | medium | long | curly | wire | hairless | double.
const COAT_TYPE_MAP = {
  wiry: "wire",
  hairless: "hairless",
  smooth: "short",
  rough: "medium",
  corded: "curly",
  double: "double",
  curly: "curly",
  wavy: "medium",
  silky: "long",
};

const COAT_LENGTH_MAP = {
  short: "short",
  medium: "medium",
  long: "long",
};

const decodeHtmlEntities = (text) => {
  if (!text) return text;
  return text
    .replace(/&quot;/g, '"')
    .replace(/&#x27;/g, "'")
    .replace(/&#39;/g, "'")
    .replace(/&rsquo;/g, "\u2019")
    .replace(/&lsquo;/g, "\u2018")
    .replace(/&ldquo;/g, "\u201C")
    .replace(/&rdquo;/g, "\u201D")
    .replace(/&mdash;/g, "\u2014")
    .replace(/&ndash;/g, "\u2013")
    .replace(/&nbsp;/g, " ")
    .replace(/&amp;/g, "&")
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">");
};

// Unescape JSON-string backslash sequences like \/ (forward slash) and \u0026 (&)
// produced when AKC embeds JSON-encoded values in HTML.
const unescapeJsonString = (text) => {
  if (!text) return text;
  return text.replace(/\\\//g, "/").replace(/\\u0026/g, "&").replace(/\\"/g, '"');
};

const mapCoatType = (coatTypeValues, coatLengthValues) => {
  if (coatTypeValues && coatTypeValues.length > 0) {
    if (coatTypeValues.includes("Hairless")) return "hairless";
    if (coatTypeValues.includes("Curly")) return "curly";
    if (coatTypeValues.includes("Wiry")) return "wire";
    const first = coatTypeValues[0].toLowerCase();
    const mapped = COAT_TYPE_MAP[first];
    if (mapped) return mapped;
  }
  if (coatLengthValues && coatLengthValues.length > 0) {
    const first = coatLengthValues[0].toLowerCase();
    const mapped = COAT_LENGTH_MAP[first];
    if (mapped) return mapped;
  }
  return null;
};

// AKC has no explicit size field, so we derive size_category from the max shoulder
// height (in inches) that appears in the page's <Attribute name="height"> value.
// Thresholds follow common canine classification: German Shepherds (~26in) are
// "large", Great Danes / Mastiffs (30in+) are "giant".
const mapSizeCategory = (heightInches) => {
  if (heightInches == null || Number.isNaN(heightInches)) return null;
  if (heightInches < 10) return "toy";
  if (heightInches < 15) return "small";
  if (heightInches < 22) return "medium";
  if (heightInches < 27) return "large";
  return "giant";
};

// Extract all numeric inch values from a height string like
// "Height: 23-24 inches (male), 21.5-22.5 inches (female)" or "Height: 30 inches & up (male)".
const extractMaxHeightInches = (heightText) => {
  if (!heightText) return null;
  const matches = heightText.match(/\d+(?:\.\d+)?/g);
  if (!matches || matches.length === 0) return null;
  const values = matches.map(Number);
  return Math.max(...values);
};

const extractSelectedValues = (html, traitKey) => {
  const pattern = new RegExp(`"${traitKey}":\\{"selected":\\[([^\\]]*)\\]`);
  const match = html.match(pattern);
  if (!match) return [];
  const raw = match[1];
  const values = [...raw.matchAll(/"([^"]+)"/g)].map((entry) => entry[1]);
  return values;
};

const extractScalarField = (html, slug, key) => {
  // Locate the breed attributes object anchored by breed_name_url, then scan a wide window.
  const anchorIndex = html.indexOf(`"breed_name_url":"${slug}"`);
  const startIndex = anchorIndex > 0 ? anchorIndex : 0;
  const chunk = html.slice(startIndex, startIndex + 12000);
  const pattern = new RegExp(`"${key}":"((?:[^"\\\\]|\\\\.)*)"`);
  const match = chunk.match(pattern);
  return match ? match[1] : null;
};

const parseDetailPage = (rawHtml, slug, fallbackName) => {
  // AKC embeds a JSON object with breed facts using HTML-entity-encoded quotes
  // (&quot;). Decode them so the regex extraction works against real quotes.
  const html = decodeHtmlEntities(rawHtml);
  const name = extractScalarField(html, slug, "breed_name") || fallbackName || slug;
  const originRaw = extractScalarField(html, slug, "origin");
  const origin = originRaw ? unescapeJsonString(originRaw) : null;
  const breedGroup = extractScalarField(html, slug, "breed_group");

  const coatLengthValues = extractSelectedValues(html, "coat_length");
  const coatTypeValues = extractSelectedValues(html, "coat_type");
  const coatType = mapCoatType(coatTypeValues, coatLengthValues);

  const heightMatch = html.match(
    /<Attribute\s+name="height">([^<]+)<\/Attribute>/i
  );
  const heightText = heightMatch ? heightMatch[1] : null;
  const maxHeightInches = extractMaxHeightInches(heightText);
  const sizeCategory = mapSizeCategory(maxHeightInches);

  return {
    id: `dog.${slug}`,
    species: "dog",
    name,
    pinyin: "",
    initial: "",
    size_category: sizeCategory,
    coat_type: coatType,
    standard_weight_min: null,
    standard_weight_max: null,
    life_span_min: null,
    life_span_max: null,
    exercise_needs: null,
    icon: null,
    origin,
  };
};

const delay = (milliseconds) =>
  new Promise((resolvePromise) => setTimeout(resolvePromise, milliseconds));

const randomDelay = () =>
  BASE_DELAY_MILLIS + Math.floor(Math.random() * EXTRA_DELAY_MILLIS);

const fetchWithRetry = async (url) => {
  let lastError = null;
  for (let attempt = 1; attempt <= MAX_RETRIES; attempt++) {
    try {
      const response = await fetch(url, {
        headers: REQUEST_HEADERS,
        redirect: "follow",
      });
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      return await response.text();
    } catch (error) {
      lastError = error;
      if (attempt < MAX_RETRIES) {
        const backoffDelay = BASE_DELAY_MILLIS * Math.pow(2, attempt - 1);
        await delay(backoffDelay);
      }
    }
  }
  throw lastError;
};

// AKC embeds a "breed_select" dropdown in every dog-breeds page listing all recognized
// breeds with their names and slugs. Fetching the list page once yields the full slug set
// without paginating through 25 pages.
const fetchAllBreedsFromDropdown = async () => {
  const rawHtml = await fetchWithRetry(AKC_LIST_URL);
  // The breed_select dropdown is JSON-encoded inside the HTML with HTML-entity-encoded
  // quotes (&quot;). Decode entities so the option regex matches plain quotes.
  const html = decodeHtmlEntities(rawHtml);
  const startIndex = html.indexOf('"breed_select"');
  if (startIndex < 0) {
    throw new Error("breed_select dropdown not found on list page");
  }
  const chunk = html.slice(startIndex, startIndex + 200000);
  const optionPattern =
    /"text":"([^"]+)","value":"https:\\?\/\\?\/www\.akc\.org\\?\/dog-breeds\\?\/([^\/"\\]+)\\?\/"/g;
  const breeds = new Map();
  let match;
  while ((match = optionPattern.exec(chunk)) !== null) {
    const name = match[1];
    const slug = match[2];
    if (!breeds.has(slug)) {
      breeds.set(slug, name);
    }
  }
  return Array.from(breeds.entries()).map(([slug, name]) => ({ slug, name }));
};

const fetchBreed = async (slug, name) => {
  const url = `${AKC_BREED_URL}${slug}/`;
  const html = await fetchWithRetry(url);
  return parseDetailPage(html, slug, name);
};

const runWithConcurrency = async (items, worker, concurrency) => {
  const results = new Array(items.length);
  let index = 0;
  let completed = 0;
  const total = items.length;

  const runWorker = async () => {
    while (true) {
      const currentIndex = index;
      index += 1;
      if (currentIndex >= total) return;

      const item = items[currentIndex];
      try {
        const result = await worker(item, currentIndex);
        results[currentIndex] = result;
      } catch (error) {
        console.error(`Failed: ${item.slug} - ${error.message}`);
        results[currentIndex] = null;
      }
      completed += 1;
      console.log(`Processing ${completed}/${total}: ${item.slug}`);
      await delay(randomDelay());
    }
  };

  const workers = Array.from({ length: concurrency }, () => runWorker());
  await Promise.all(workers);
  return results;
};

const main = async () => {
  console.log("Fetching AKC breed list...");
  const breeds = await fetchAllBreedsFromDropdown();
  console.log(`AKC breeds to fetch: ${breeds.length}`);

  await mkdir(dirname(outputPath), { recursive: true });

  const results = await runWithConcurrency(
    breeds,
    (entry) => fetchBreed(entry.slug, entry.name),
    CONCURRENCY_LIMIT
  );

  const validBreeds = results.filter((breed) => breed !== null);
  const failedSlugs = breeds
    .filter((_, index) => results[index] === null)
    .map((entry) => entry.slug);

  const uniqueBreeds = new Map();
  for (const breed of validBreeds) {
    if (!uniqueBreeds.has(breed.id)) {
      uniqueBreeds.set(breed.id, breed);
    }
  }

  const finalBreeds = Array.from(uniqueBreeds.values()).sort((first, second) =>
    first.id.localeCompare(second.id)
  );

  const jsonString = JSON.stringify(finalBreeds, null, 2);
  await writeFile(outputPath, jsonString, "utf8");

  const coatCoverage = finalBreeds.filter((breed) => breed.coat_type).length;
  const sizeCoverage = finalBreeds.filter((breed) => breed.size_category).length;
  const originCoverage = finalBreeds.filter((breed) => breed.origin).length;

  console.log("\n=== Summary ===");
  console.log(`Total breeds: ${finalBreeds.length}`);
  console.log(`Coat type coverage: ${coatCoverage}/${finalBreeds.length}`);
  console.log(`Size category coverage: ${sizeCoverage}/${finalBreeds.length}`);
  console.log(`Origin coverage: ${originCoverage}/${finalBreeds.length}`);
  if (failedSlugs.length > 0) {
    console.log(`Failed slugs (${failedSlugs.length}): ${failedSlugs.join(", ")}`);
  }
  console.log(`Output written to: ${outputPath}`);
};

main().catch((error) => {
  console.error(`Fatal error: ${error.message}`);
  process.exitCode = 1;
});
