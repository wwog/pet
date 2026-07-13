const getCrateLatestVersion = async (name) => {
  return fetch(`https://crates.io/api/v1/crates/${name}`, {
    headers: {
      "User-Agent":
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36",
      accept:
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
      "accept-language": "zh-CN,zh;q=0.9,en;q=0.8",
      "cache-control": "no-cache",
      pragma: "no-cache",
      priority: "u=0, i",
      "sec-ch-ua":
        '"Chromium";v="146", "Not-A.Brand";v="24", "Google Chrome";v="146"',
      "sec-ch-ua-mobile": "?0",
      "sec-ch-ua-platform": '"macOS"',
      "sec-fetch-dest": "document",
      "sec-fetch-mode": "navigate",
      "sec-fetch-site": "none",
      "sec-fetch-user": "?1",
      "upgrade-insecure-requests": "1",
    },
  })
    .then((response) => response.json())
    .then((data) => {
      return data?.crate?.default_version;
    })
    .catch((error) => {
      console.error(error);
      return null;
    });
};

// node scripts/getCrateVersion.js [crate-name]
const main = async () => {
  //获取 args
  const args = process.argv.slice(2);
  const name = args[0];
  const version = await getCrateLatestVersion(name);
  console.log(`${name} latest version:${version ?? "*"}`);
};

main();
