const { readFileSync } = require("fs");

(async () => {
  console.log("🚀 New Update Available!");

  const octokit = await import("octokit");
  const { Octokit } = octokit;

  const client = new Octokit({
    auth: process.env.GH_TOKEN,
  });

  const release = await client.rest.repos.createRelease({
    owner: "ahqstore",
    repo: "redistributed_apps",
    tag_name: `vlc_${Date.now()}`,
    name: "VLC new version",
  });

  const {
    data: { browser_download_url },
  } = await client.rest.repos.uploadReleaseAsset({
    owner: "ahqstore",
    repo: "redistributed_apps",
    release_id: release.data.id,
    name: "vlc.txt",
    data: readFileSync("./vlc.json").toString(),
  });

  await client.rest.issues.createComment({
    owner: "ahqstore",
    repo: "apps",
    issue_number: 3,
    body: `/store set ${browser_download_url}`,
  });
})();
