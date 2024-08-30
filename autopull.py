import subprocess
import requests
import time
from datetime import datetime

# GitHub API URL
repo_url = "https://api.github.com/repos/liyang8246/pcywwxzx-backend"

def get_last_commit_date():
    global last_commit_date
    response = requests.get(repo_url)
    response.raise_for_status()
    latest_commit = response.json()["pushed_at"]
    return datetime.strptime(latest_commit, "%Y-%m-%dT%H:%M:%SZ")

last_commit_date = datetime.fromtimestamp(0)
process = subprocess.Popen(["cargo", "run", "--release"])

while True:
    commit_date = get_last_commit_date()
    if last_commit_date < commit_date:
        subprocess.run(["git", "pull"])
        last_commit_date = commit_date
    else:
        time.sleep(60)
        continue
    process.terminate()
    process.wait()
    process = subprocess.Popen(["cargo", "run", "--release"])