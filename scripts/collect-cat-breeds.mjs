import { mkdir, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const directoryPath = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const outputPath = resolve(directoryPath, "data", "breeds", "cats.json");

const CFA_BASE_URL = "https://cfa.org/breed/";

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

const CFA_BREED_SLUGS = [
  "abyssinian",
  "american-bobtail",
  "american-curl",
  "american-shorthair",
  "american-wirehair",
  "balinese",
  "bengal",
  "birman",
  "bombay",
  "british-shorthair",
  "burmese",
  "burmilla",
  "chartreux",
  "colorpoint-shorthair",
  "cornish-rex",
  "devon-rex",
  "egyptian-mau",
  "european-burmese",
  "exotic",
  "havana-brown",
  "japanese-bobtail",
  "khao-manee",
  "korat",
  "laperm",
  "lykoi",
  "maine-coon-cat",
  "manx",
  "norwegian-forest-cat",
  "ocicat",
  "oriental",
  "persian",
  "ragamuffin",
  "ragdoll",
  "russian-blue",
  "scottish-fold",
  "selkirk-rex",
  "siamese",
  "siberian",
  "singapura",
  "somali",
  "sphynx",
  "tonkinese",
  "toybob",
  "turkish-angora",
  "turkish-van",
];

const slugToName = (slug) => {
  const specialNames = {
    "maine-coon-cat": "Maine Coon",
    ragamuffin: "RagaMuffin",
  };
  if (specialNames[slug]) return specialNames[slug];
  return slug
    .split("-")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
};

const slugFromName = (name) =>
  name.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");

// Sphynx is listed as "Shorthair" by CFA but is functionally hairless;
// the breed name itself is the reliable signal.
const HAIRLESS_SLUGS = new Set(["sphynx", "lykoi"]);
const WIRE_SLUGS = new Set(["american-wirehair"]);
const CURLY_SLUGS = new Set(["cornish-rex", "devon-rex", "selkirk-rex", "laperm"]);

const mapCoatType = (coatText, slug, characteristics) => {
  if (HAIRLESS_SLUGS.has(slug)) return "hairless";
  if (WIRE_SLUGS.has(slug)) return "wire";
  if (CURLY_SLUGS.has(slug)) return "curly";
  const combined = `${coatText || ""} ${characteristics || ""}`.toLowerCase();
  if (combined.includes("hairless")) return "hairless";
  if (!coatText) return null;
  const normalized = coatText.toLowerCase();
  if (normalized.includes("wire")) return "wire";
  if (normalized.includes("curly") || normalized.includes("rex")) return "curly";
  if (normalized.includes("long") && normalized.includes("short")) return "medium";
  if (normalized.includes("long")) return "long";
  if (normalized.includes("short")) return "short";
  return null;
};

// Size categories per CFA breed standard (manually verified from each
// breed standard's "Size" / "General" description on cfa.org).
const CFA_SIZE_CATEGORIES = {
  abyssinian: "medium",
  "american-bobtail": "large",
  "american-curl": "medium",
  "american-shorthair": "medium",
  "american-wirehair": "medium",
  balinese: "medium",
  bengal: "large",
  birman: "medium",
  bombay: "medium",
  "british-shorthair": "medium",
  burmese: "medium",
  burmilla: "medium",
  chartreux: "medium",
  "colorpoint-shorthair": "medium",
  "cornish-rex": "medium",
  "devon-rex": "medium",
  "egyptian-mau": "medium",
  "european-burmese": "medium",
  exotic: "medium",
  "havana-brown": "medium",
  "japanese-bobtail": "medium",
  "khao-manee": "medium",
  korat: "medium",
  laperm: "medium",
  lykoi: "medium",
  "maine-coon-cat": "large",
  manx: "medium",
  "norwegian-forest-cat": "large",
  ocicat: "large",
  oriental: "medium",
  persian: "medium",
  ragamuffin: "large",
  ragdoll: "large",
  "russian-blue": "medium",
  "scottish-fold": "medium",
  "selkirk-rex": "large",
  siamese: "medium",
  siberian: "large",
  singapura: "small",
  somali: "medium",
  sphynx: "medium",
  tonkinese: "medium",
  toybob: "small",
  "turkish-angora": "medium",
  "turkish-van": "large",
};

const mapSizeCategory = (slug) => CFA_SIZE_CATEGORIES[slug] || null;

const ORIGIN_KEYWORD_MAP = [
  { keyword: "ethiopia", origin: "Ethiopia" },
  { keyword: "abyssinia", origin: "Ethiopia" },
  { keyword: "persia", origin: "Iran (Persia)" },
  { keyword: "iran", origin: "Iran" },
  { keyword: "thailand", origin: "Thailand" },
  { keyword: "siam", origin: "Thailand" },
  { keyword: "singapore", origin: "Singapore" },
  { keyword: "japan", origin: "Japan" },
  { keyword: "egypt", origin: "Egypt" },
  { keyword: "russia", origin: "Russia" },
  { keyword: "siberia", origin: "Russia" },
  { keyword: "norway", origin: "Norway" },
  { keyword: "turkey", origin: "Turkey" },
  { keyword: "anhara", origin: "Turkey" },
  { keyword: "burma", origin: "Myanmar (Burma)" },
  { keyword: "france", origin: "France" },
  { keyword: "britain", origin: "United Kingdom" },
  { keyword: "british", origin: "United Kingdom" },
  { keyword: "england", origin: "United Kingdom" },
  { keyword: "scotland", origin: "United Kingdom" },
  { keyword: "canada", origin: "Canada" },
  { keyword: "toronto", origin: "Canada" },
  { keyword: "united states", origin: "United States" },
  { keyword: "america", origin: "United States" },
  { keyword: "u.s.", origin: "United States" },
  { keyword: "kentucky", origin: "United States" },
  { keyword: "california", origin: "United States" },
  { keyword: "new england", origin: "United States" },
  { keyword: "maine", origin: "United States" },
  { keyword: "indonesia", origin: "Indonesia" },
  { keyword: "australia", origin: "Australia" },
  { keyword: "germany", origin: "Germany" },
  { keyword: "italy", origin: "Italy" },
  { keyword: "netherlands", origin: "Netherlands" },
  { keyword: "cuba", origin: "Cuba" },
  { keyword: "china", origin: "China" },
  { keyword: "india", origin: "India" },
  { keyword: "ireland", origin: "Ireland" },
  { keyword: "wales", origin: "United Kingdom" },
];

const NAME_ORIGIN_HINTS = {
  abyssinian: "Ethiopia",
  "egyptian-mau": "Egypt",
  "japanese-bobtail": "Japan",
  "korat": "Thailand",
  "khao-manee": "Thailand",
  "norwegian-forest-cat": "Norway",
  "russian-blue": "Russia",
  "siberian": "Russia",
  "singapura": "Singapore",
  "siamese": "Thailand",
  "somali": "Ethiopia",
  "turkish-angora": "Turkey",
  "turkish-van": "Turkey",
  "persian": "Iran (Persia)",
  "birman": "Myanmar (Burma)",
  "burmese": "Myanmar (Burma)",
  "european-burmese": "Myanmar (Burma)",
  "british-shorthair": "United Kingdom",
  "scottish-fold": "United Kingdom",
  "bombay": "United States",
  "havana-brown": "United Kingdom",
  "chartreux": "France",
  "ocicat": "United States",
  "ragdoll": "United States",
  "ragamuffin": "United States",
  "exotic": "United States",
  "american-bobtail": "United States",
  "american-curl": "United States",
  "american-shorthair": "United States",
  "american-wirehair": "United States",
  "maine-coon-cat": "United States",
  "manx": "United Kingdom",
  "colorpoint-shorthair": "United States",
  "oriental": "United States",
  "balinese": "United States",
  "bengal": "United States",
  "burmilla": "United Kingdom",
  "cornish-rex": "United Kingdom",
  "devon-rex": "United Kingdom",
  "selkirk-rex": "United States",
  "laperm": "United States",
  "lykoi": "United States",
  "sphynx": "Canada",
  "tonkinese": "Canada",
  "toybob": "Russia",
};

const extractOrigin = (slug, breedHistory, characteristics) => {
  // Name-based hints are manually verified per-breed and take precedence
  // over free-text keyword matching which can misattribute origin (e.g.,
  // Maine Coon history mentioning "British" settlers).
  const hint = NAME_ORIGIN_HINTS[slug];
  if (hint) return hint;

  const searchText = `${breedHistory || ""} ${characteristics || ""}`;
  for (const { keyword, origin } of ORIGIN_KEYWORD_MAP) {
    const regex = new RegExp(`\\b${keyword.replace(/\./g, "\\.")}\\b`, "i");
    if (regex.test(searchText)) {
      return origin;
    }
  }
  return null;
};

const decodeHtmlEntities = (text) => {
  if (!text) return text;
  return text
    .replace(/&rsquo;/g, "\u2019")
    .replace(/&lsquo;/g, "\u2018")
    .replace(/&ldquo;/g, "\u201C")
    .replace(/&rdquo;/g, "\u201D")
    .replace(/&mdash;/g, "\u2014")
    .replace(/&ndash;/g, "\u2013")
    .replace(/&nbsp;/g, " ")
    .replace(/&amp;/g, "&")
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&#8217;/g, "\u2019")
    .replace(/&#8220;/g, "\u201C")
    .replace(/&#8221;/g, "\u201D")
    .replace(/&#8230;/g, "\u2026");
};

const stripHtmlTags = (html) => {
  return html
    .replace(/<script[\s\S]*?<\/script>/gi, "")
    .replace(/<style[\s\S]*?<\/style>/gi, "")
    .replace(/<[^>]+>/g, " ")
    .replace(/\s+/g, " ")
    .trim();
};

const parseDetailPage = (html, slug) => {
  const name = slugToName(slug);

  const coatLengthMatch = html.match(
    /<div[^>]*class="h6"[^>]*>\s*Coat Length:\s*<\/div>\s*<p>([^<]+)<\/p>/i
  );
  const coatLengthText = coatLengthMatch
    ? decodeHtmlEntities(coatLengthMatch[1].trim())
    : null;

  const characteristicsMatch = html.match(
    /<div[^>]*class="h6"[^>]*>\s*Characteristics:\s*<\/div>\s*<p>([^<]+)<\/p>/i
  );
  const characteristicsText = characteristicsMatch
    ? decodeHtmlEntities(characteristicsMatch[1].trim())
    : null;

  const breedHistoryMatch = html.match(
    /<div[^>]*class="h2"[^>]*>\s*Breed History\s*<\/div>\s*<div[^>]*class="description"[^>]*>\s*<p>([\s\S]*?)<\/p>/i
  );
  let breedHistoryText = "";
  if (breedHistoryMatch) {
    breedHistoryText = decodeHtmlEntities(stripHtmlTags(breedHistoryMatch[1]));
  }

  const coatType = mapCoatType(coatLengthText, slug, characteristicsText);
  const sizeCategory = mapSizeCategory(slug);
  const origin = extractOrigin(slug, breedHistoryText, characteristicsText);

  return {
    id: `cat.${slug}`,
    species: "cat",
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
    _source: { coatLengthText, characteristicsText },
  };
};

const delay = (milliseconds) => new Promise((resolvePromise) => setTimeout(resolvePromise, milliseconds));

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

const fetchBreed = async (slug) => {
  const url = `${CFA_BASE_URL}${slug}/`;
  const html = await fetchWithRetry(url);
  return parseDetailPage(html, slug);
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
        console.error(`Failed: ${item} - ${error.message}`);
        results[currentIndex] = null;
      }
      completed += 1;
      console.log(`Processing ${completed}/${total}: ${item}`);
      await delay(randomDelay());
    }
  };

  const workers = Array.from({ length: concurrency }, () => runWorker());
  await Promise.all(workers);
  return results;
};

const main = async () => {
  console.log(`CFA breeds to fetch: ${CFA_BREED_SLUGS.length}`);

  await mkdir(dirname(outputPath), { recursive: true });

  const results = await runWithConcurrency(
    CFA_BREED_SLUGS,
    async (slug) => fetchBreed(slug),
    CONCURRENCY_LIMIT
  );

  const validBreeds = results.filter((breed) => breed !== null);
  const failedSlugs = CFA_BREED_SLUGS.filter(
    (_, index) => results[index] === null
  );

  const cleanedBreeds = validBreeds.map((breed) => {
    const { _source, ...publicFields } = breed;
    return publicFields;
  });

  const uniqueBreeds = new Map();
  for (const breed of cleanedBreeds) {
    if (!uniqueBreeds.has(breed.id)) {
      uniqueBreeds.set(breed.id, breed);
    }
  }

  const finalBreeds = Array.from(uniqueBreeds.values());

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
    console.log(`Failed slugs: ${failedSlugs.join(", ")}`);
  }
  console.log(`Output written to: ${outputPath}`);
};

main().catch((error) => {
  console.error(`Fatal error: ${error.message}`);
  process.exitCode = 1;
});
