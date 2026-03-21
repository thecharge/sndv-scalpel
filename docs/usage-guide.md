# Usage Guide

This guide is auto-generated from a shell test suite run against real CLI commands.
## 1. Find across fixture languages

Example files:
- [tests/fixtures/sample.js](../tests/fixtures/sample.js)
- [tests/fixtures/sample.ts](../tests/fixtures/sample.ts)
- [tests/fixtures/sample.go](../tests/fixtures/sample.go)
- [tests/fixtures/sample.rs](../tests/fixtures/sample.rs)

Command:

```bash
scalpel find 'fn:*' tests/fixtures --recursive
```

Actual output:

```text
/tmp/scalpel-work/fixtures/sample-complex.ts:73-76 [typescript:1:structural] function formatSummary
/tmp/scalpel-work/fixtures/sample.lua:2-12 [lua:1:structural] function calculate_total
/tmp/scalpel-work/fixtures/sample.lua:10-12 [lua:1:structural] function M.run
/tmp/scalpel-work/fixtures/sample.rs:4-11 [rust:1:structural] function calculate_total
/tmp/scalpel-work/fixtures/sample.rs:14-14 [rust:1:structural] function run
/tmp/scalpel-work/fixtures/sample.js:2-9 [javascript:1:structural] function calculateTotal
/tmp/scalpel-work/fixtures/big/large-service.go:4-1508 [go:1:structural] function CalculateTotal
/tmp/scalpel-work/fixtures/sample.ts:4-7 [typescript:1:structural] function buildReport
/tmp/scalpel-work/fixtures/big/large-service.rs:4-1507 [rust:1:structural] function process_order
/tmp/scalpel-work/fixtures/sample-import-groups.go:7-11 [go:1:structural] function Run
/tmp/scalpel-work/fixtures/sample.go:6-13 [go:1:structural] function CalculateTotal
```

## 2. View matched function with context

Example files:
- [tests/fixtures/sample.go](../tests/fixtures/sample.go)

Command:

```bash
scalpel view 'fn:CalculateTotal' tests/fixtures/sample.go --context 2
```

Actual output:

```text
--- /tmp/scalpel-work/sample.go:6-13 ---
    4 | 
    5 | type Config struct{}
    6 | 
    7 | func CalculateTotal(items []int) int {
    8 |     sum := 0
    9 |     for _, item := range items {
   10 |         sum += item
   11 |     }
   12 |     return sum
   13 | }
   14 | 
   15 | func (c *Config) Run() {
```

## 3. View structural outline

Example files:
- [tests/fixtures/sample.go](../tests/fixtures/sample.go)

Command:

```bash
scalpel view tests/fixtures/sample.go --outline
```

Actual output:

```text
file: /tmp/scalpel-work/sample.go
language: go
- import fmt:2-2
- type Config:4-4
  - method Run:14-17
- function CalculateTotal:6-13
```

## 4. View explicit line window

Example files:
- [tests/fixtures/sample.rs](../tests/fixtures/sample.rs)

Command:

```bash
scalpel view tests/fixtures/sample.rs --lines 1:6
```

Actual output:

```text
--- /tmp/scalpel-work/sample.rs:1-6 ---
    1 | use std::fmt::Display;
    2 | 
    3 | pub struct Worker;
    4 | 
    5 | pub fn calculate_total(items: &[i32]) -> i32 {
    6 |     let mut sum = 0;
```

## 5. Preview rename diff (dry-run)

Example files:
- [tests/fixtures/sample.go](../tests/fixtures/sample.go)

Command:

```bash
scalpel diff 'fn:CalculateTotal' tests/fixtures/sample.go --rename sum=total
```

Actual output:

```text
--- a//tmp/scalpel-work/sample.go
+++ b//tmp/scalpel-work/sample.go
@@ -5,11 +5,11 @@
 type Config struct{}
 
 func CalculateTotal(items []int) int {
-    sum := 0
+    total := 0
     for _, item := range items {
-        sum += item
+        total += item
     }
-    return sum
+    return total
 }
 
 func (c *Config) Run() {
dry-run only. pass --apply to write changes.
```

## 6. Apply transactional rename patch

Example files:
- [tests/fixtures/sample.rs](../tests/fixtures/sample.rs)

Command:

```bash
scalpel patch 'fn:calculate_total' tests/fixtures/sample.rs --rename sum=total --apply
```

Actual output:

```text
--- a//tmp/scalpel-work/sample.rs
+++ b//tmp/scalpel-work/sample.rs
@@ -3,11 +3,11 @@
 pub struct Worker;
 
 pub fn calculate_total(items: &[i32]) -> i32 {
-    let mut sum = 0;
+    let mut total = 0;
     for item in items {
-        sum += item;
+        total += item;
     }
-    sum
+    total
 }
 
 impl Worker {
applied: /tmp/scalpel-work/sample.rs
```

## 7. Swap direct line range

Example files:
- [tests/fixtures/sample.txt](../tests/fixtures/sample.txt)

Command:

```bash
scalpel patch '*' tests/fixtures/sample.txt --from-line 2 --to-line 2 --body "$(printf 'status: running\n')" --apply
```

Actual output:

```text
--- a//tmp/scalpel-work/sample.txt
+++ b//tmp/scalpel-work/sample.txt
@@ -1,3 +1,2 @@
 title: Scalpel Notes
-status: queued
-owner: team-a
+status: runningowner: team-a
applied: /tmp/scalpel-work/sample.txt
```

## 8. Swap Go function body from file

Example files:
- [tests/fixtures/sample.go](../tests/fixtures/sample.go)

Command:

```bash
scalpel patch 'fn:CalculateTotal' tests/fixtures/sample.go --body-file /tmp/new-total.go --apply
```

Actual output:

```text
--- a//tmp/scalpel-work/sample.go
+++ b//tmp/scalpel-work/sample.go
@@ -3,15 +3,12 @@
 import "fmt"
 
 type Config struct{}
-
 func CalculateTotal(items []int) int {
-    sum := 0
-    for _, item := range items {
-        sum += item
-    }
-    return sum
+    total := 100
+    return total
 }
 
+
 func (c *Config) Run() {
     fmt.Println("run")
 }
applied: /tmp/scalpel-work/sample.go
```

## 9. Swap grouped Go imports

Example files:
- [tests/fixtures/sample-import-groups.go](../tests/fixtures/sample-import-groups.go)

Command:

```bash
scalpel patch 'import:import' tests/fixtures/sample-import-groups.go --body-file /tmp/new-imports.go.frag --apply
```

Actual output:

```text
--- a//tmp/scalpel-work/sample-import-groups.go
+++ b//tmp/scalpel-work/sample-import-groups.go
@@ -1,10 +1,10 @@
 package sample
-
 import (
-	"fmt"
-	"strings"
+    "strings"
+    "fmt"
 )
 
+
 func Run(value string) string {
     fmt.Println(value)
     return strings.TrimSpace(value)
applied: /tmp/scalpel-work/sample-import-groups.go
```

## 10. Scoped JSON key replacement

Example files:
- [tests/fixtures/sample.json](../tests/fixtures/sample.json)

Command:

```bash
scalpel patch 'key:service.mode' tests/fixtures/sample.json --replace 'safe=>strict' --apply
```

Actual output:

```text
--- a//tmp/scalpel-work/sample.json
+++ b//tmp/scalpel-work/sample.json
@@ -1 +1 @@
-{"service":{"name":"scalpel","mode":"safe"},"limits":{"max_file_bytes":33554432}}
\ No newline at end of file
+{"service":{"name":"scalpel","mode":"strict"},"limits":{"max_file_bytes":33554432}}
\ No newline at end of file
applied: /tmp/scalpel-work/sample.json
```

## 11. Structured JSON output from view

Example files:
- [tests/fixtures/sample.rs](../tests/fixtures/sample.rs)

Command:

```bash
scalpel --json view 'fn:calculate_total' tests/fixtures/sample.rs
```

Actual output:

```text
{
  "path": "/tmp/scalpel-work/sample.rs",
  "language": "rust",
  "mode": "structural",
  "tier": 1,
  "pattern": "fn:calculate_total",
  "symbol": {
    "file": "/tmp/scalpel-work/sample.rs",
    "kind": "function",
    "name": "calculate_total",
    "start_line": 4,
    "end_line": 11,
    "start_byte": 43,
    "end_byte": 178,
    "signature": "pub fn calculate_total("
  },
  "lines": [
    {
      "number": 1,
      "text": "use std::fmt::Display;"
    },
    {
      "number": 2,
      "text": ""
    },
    {
      "number": 3,
      "text": "pub struct Worker;"
    },
    {
      "number": 4,
      "text": ""
    },
    {
      "number": 5,
      "text": "pub fn calculate_total(items: &[i32]) -> i32 {"
    },
    {
      "number": 6,
      "text": "    let mut total = 0;"
    },
    {
      "number": 7,
      "text": "    for item in items {"
    },
    {
      "number": 8,
      "text": "        total += item;"
    },
    {
      "number": 9,
      "text": "    }"
    },
    {
      "number": 10,
      "text": "    total"
    },
    {
      "number": 11,
      "text": "}"
    },
    {
      "number": 12,
      "text": ""
    },
    {
      "number": 13,
      "text": "impl Worker {"
    },
    {
      "number": 14,
      "text": "    pub fn run(&self) {}"
    }
  ]
}
```

## 12. Structured JSON output from diff

Example files:
- [tests/fixtures/sample.go](../tests/fixtures/sample.go)

Command:

```bash
scalpel --json diff 'fn:CalculateTotal' tests/fixtures/sample.go --rename sum=total
```

Actual output:

```text
{
  "path": "/tmp/scalpel-work/sample.go",
  "applied": false,
  "dry_run": true,
  "changed": false,
  "diff": ""
}
```

## 13. Peek paginated content

Example files:
- [tests/fixtures/sample.go](../tests/fixtures/sample.go)

Command:

```bash
scalpel peek tests/fixtures/sample.go --page-size 5 --page 1
```

Actual output:

```text
--- /tmp/scalpel-work/sample.go:1-5 (total: 14) ---
    1 | package sample
    2 | 
    3 | import "fmt"
    4 | 
    5 | type Config struct{}
next page: --page 2
```

## 14. Generate bash completion script

Example files:
- [README.md](../README.md)

Command:

```bash
scalpel completion bash
```

Actual output:

```text
_scalpel() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="scalpel"
                ;;
            scalpel,completion)
                cmd="scalpel__completion"
                ;;
            scalpel,diff)
                cmd="scalpel__diff"
                ;;
            scalpel,find)
                cmd="scalpel__find"
                ;;
            scalpel,help)
                cmd="scalpel__help"
                ;;
            scalpel,info)
                cmd="scalpel__info"
                ;;
            scalpel,patch)
                cmd="scalpel__patch"
                ;;
            scalpel,peek)
                cmd="scalpel__peek"
                ;;
            scalpel,view)
                cmd="scalpel__view"
                ;;
            scalpel__help,completion)
                cmd="scalpel__help__completion"
                ;;
            scalpel__help,diff)
                cmd="scalpel__help__diff"
                ;;
            scalpel__help,find)
                cmd="scalpel__help__find"
                ;;
            scalpel__help,help)
                cmd="scalpel__help__help"
                ;;
            scalpel__help,info)
                cmd="scalpel__help__info"
                ;;
            scalpel__help,patch)
                cmd="scalpel__help__patch"
                ;;
            scalpel__help,peek)
                cmd="scalpel__help__peek"
                ;;
            scalpel__help,view)
                cmd="scalpel__help__view"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        scalpel)
            opts="-h -V --json --concurrency --config --help --version find view peek info diff patch completion help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --concurrency)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__completion)
            opts="-h --json --concurrency --config --help bash elvish fish powershell zsh"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --concurrency)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__diff)
            opts="-h --rename --replace --body --body-file --index --from-line --to-line --json --concurrency --config --help <PATTERN> <PATH>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --rename)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --replace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --body)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --body-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --index)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --from-line)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --to-line)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --concurrency)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__find)
            opts="-r -h --recursive --json --concurrency --config --help <PATTERN> <PATHS>..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --concurrency)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__help)
            opts="find view peek info diff patch completion help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__help__completion)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__help__diff)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__help__find)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__help__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__help__patch)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__help__peek)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__help__view)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__info)
            opts="-h --json --concurrency --config --help <PATH>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --concurrency)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__patch)
            opts="-h --rename --replace --body --body-file --apply --index --from-line --to-line --json --concurrency --config --help <PATTERN> <PATH>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --rename)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --replace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --body)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --body-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --index)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --from-line)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --to-line)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --concurrency)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__peek)
            opts="-h --from-pos --from-line --to-pos --to-line --page-size --page --all --json --concurrency --config --help <PATH>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --from-line)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --from-pos)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --to-line)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --to-pos)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --page-size)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --page)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --concurrency)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        scalpel__view)
            opts="-h --context --index --outline --lines --all --json --concurrency --config --help <PATTERN_OR_PATH> [PATH]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --context)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --index)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --lines)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --concurrency)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _scalpel -o nosort -o bashdefault -o default scalpel
else
    complete -F _scalpel -o bashdefault -o default scalpel
fi
```

