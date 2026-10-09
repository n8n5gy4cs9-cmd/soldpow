#!/usr/bin/env python3
"""Task tracker for nordSoldPow.  PRD.json is the source of truth; task.md is generated.

  python3 tools/task.py sync                 # regenerate task.md
  python3 tools/task.py done 3.1 3.2 3.3     # mark done
  python3 tools/task.py doing 4.1            # mark in progress
  python3 tools/task.py todo 4.1             # reset
  python3 tools/task.py note 4.1 "text"      # attach a note (what is left / gotchas)
  python3 tools/task.py list                 # show open tasks
"""
import json, sys, datetime, pathlib
ROOT = pathlib.Path(__file__).resolve().parent.parent
PRD = ROOT / "PRD.json"
MARK = {"todo": "[ ]", "doing": "[~]", "done": "[x]", "blocked": "[!]"}

def load(): return json.loads(PRD.read_text())
def save(d):
    d["last_updated"] = datetime.date.today().isoformat()
    PRD.write_text(json.dumps(d, indent=1) + "\n")
    sync(d)

def sync(d=None):
    d = d or load()
    t = d["tasks"]; total = len(t); done = sum(x["status"] == "done" for x in t)
    req = [x for x in t if not x["optional"]]; rdone = sum(x["status"] == "done" for x in req)
    out = [f"# nordSoldPow - TASKS\n",
           f"_Generated from PRD.json by `python3 tools/task.py sync` - do not hand-edit; use `tools/task.py done <id>`._\n",
           f"**Progress: {done}/{total} total | {rdone}/{len(req)} required | updated {d['last_updated']}**\n",
           "Legend: `[x]` done, `[~]` in progress, `[ ]` to do, `[!]` blocked, *(opt)* optional.\n",
           "**Continue here: see HANDOFF.md**\n"]
    nxt = [x for x in t if x["status"] in ("doing", "todo") and not x["optional"]][:5]
    if nxt:
        out.append("## NEXT UP\n")
        out += [f"- `{x['id']}` {x['title']}" for x in nxt]; out.append("")
    for p in d["phases"]:
        pt = [x for x in t if x["phase"] == p["id"]]
        pd = sum(x["status"] == "done" for x in pt)
        out.append(f"## {p['id']} {p['title']}  ({pd}/{len(pt)})\n")
        for x in pt:
            o = " *(opt)*" if x["optional"] else ""
            n = f" - _{x['notes']}_" if x["notes"] else ""
            out.append(f"- {MARK[x['status']]} `{x['id']}` {x['title']}{o}{n}")
        out.append("")
    (ROOT / "task.md").write_text("\n".join(out))

def main():
    a = sys.argv[1:]
    if not a or a[0] == "sync": return sync()
    d = load(); idx = {x["id"]: x for x in d["tasks"]}
    if a[0] == "list":
        for x in d["tasks"]:
            if x["status"] != "done": print(MARK[x["status"]], x["id"], x["title"], "(opt)" if x["optional"] else "")
        return
    if a[0] in ("done", "doing", "todo", "blocked"):
        for i in a[1:]:
            if i not in idx: sys.exit(f"unknown task {i}")
            idx[i]["status"] = a[0]
    elif a[0] == "note":
        idx[a[1]]["notes"] = a[2]
    else: sys.exit(__doc__)
    save(d)
main()
