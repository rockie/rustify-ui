#!/usr/bin/env bash
# 把 出仓 skill 从 .agents/skills/ 导出到公开 skills 仓（XGENT-ai/skills 的 skills/）。
#
#   scripts/export-skills.sh <目标目录>     导出（先体检，全过才写盘）
#   scripts/export-skills.sh --check        只体检，不写盘（不需要目标目录）
#
# 出仓件是**公开**的（`npx skills add XGENT-ai/skills` 谁都能装），所以导出前先体检一遍
# **暂存树** —— 也就是真正要拷过去的那些字节，而不是源目录：exclude 掉的东西不该体检，
# 体检过的东西必须原样落地。发现密钥/线上环境信息/固定 skill 安装路径就整体拒绝，不做部分导出。
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SRC="$ROOT/.agents/skills"

# 本仓维护并对外提供的 skill；其他开发辅助 skill 不导出。
SKILLS=(use-rustify-docs)

# evals/ 不出仓：评测夹具仅供仓内验证使用。
EXCLUDE_DIRS=(evals)
# 这些名字出现在 skill 目录里就是【填好的真配置】而不是模板，直接拒绝导出。
FORBIDDEN=(registry.env .xgent-registry.env .env .env.local id_rsa id_ed25519)

# ── 体检规则：<标签>|<ERE> ───────────────────────────────────────────────────
# 只放**形状能判定**的东西。判不了形状的（比如"这句话算不算内部信息"）靠人看，不靠正则。
RULES=(
  "skill 固定安装路径|\.[[:alnum:]_-]+/skills/"
  "私钥块|-----BEGIN [A-Za-z ]*PRIVATE KEY"
  "AWS access key|AKIA[0-9A-Z]{16}"
  "GitHub 令牌|gh[pousr]_[A-Za-z0-9]{20,}"
  "模型厂商 API key|sk-(ant-)?[A-Za-z0-9_-]{20,}"
  "平台令牌真值|x(rel|pms|isk)_[A-Za-z0-9]{16,}"
  "密钥赋值|[A-Z][A-Z0-9_]*(SECRET|PASSWORD|PASSWD|TOKEN|API_?KEY|_KEY|_AUTH)[[:space:]]*=[[:space:]]*['\"]?[A-Za-z0-9/+=_.-]{8,}"
  "大块 base64/hex|([A-Za-z0-9+/]{40,}={1,2}|[0-9a-f]{40,})"
  "线上主机名|[a-z0-9-]+\.(supagent\.cn|xgent\.ai|aliyuncs\.com|myqcloud\.com|tencentcloudcr\.com|amazonaws\.com)"
  "IP 地址|([0-9]{1,3}\.){3}[0-9]{1,3}"
  "邮箱|[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+\.[A-Za-z]{2,}"
  "robot 账号名|robot\\\$[A-Za-z0-9_-]+\+[A-Za-z0-9_-]+"
)

# ── 豁免表：<相对路径 或 *>::<对命中文本的 ERE>::<理由> ─────────────────────
# ⚠️ 只放**夹具/占位或已核对的公开标识**，绝不放真密钥。每条都要写理由 —— 命中时打印，
#    看不见的豁免等于没有体检。
ALLOW=(
  "*::^(127\.0\.0\.1|0\.0\.0\.0|255\.255\.255\.255)\$::本机/通配地址，不是线上环境"
  "use-rustify-docs/references/rustify-docs.md::^032756c041615dca075a49685fbb458ce7cc833f\$::作者契约引用的公开 Rustify 源码提交号，不是密钥"
)

c_red=$'\033[31m'; c_grn=$'\033[32m'; c_dim=$'\033[2m'; c_off=$'\033[0m'
info() { printf '%s>%s %s\n' "$c_dim" "$c_off" "$*"; }
die()  { printf '%sERROR: %s%s\n' "$c_red" "$*" "$c_off" >&2; exit 1; }

CHECK_ONLY=0; DST=""
while [ $# -gt 0 ]; do
  case "$1" in
    --check) CHECK_ONLY=1; shift ;;
    -h|--help) sed -n '2,8p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 0 ;;
    -*) die "不认识的参数 '$1'" ;;
    *) DST="$1"; shift ;;
  esac
done
[ "$CHECK_ONLY" = 1 ] || [ -n "$DST" ] || die "要一个目标目录：$0 <目标目录>（例：…/skills/skills）。只想体检用 $0 --check"

# ── 1) 暂存 ────────────────────────────────────────────────────────────────
STAGE="$(mktemp -d)"; trap 'rm -rf "$STAGE"' EXIT
for s in "${SKILLS[@]}"; do
  [ -d "$SRC/$s" ] || die "$SRC/$s 不存在 —— 名单里的 skill 改名或删了？"
  [ -f "$SRC/$s/SKILL.md" ] || die "$s 没有 SKILL.md，这不是一个 skill 目录"
  for f in "${FORBIDDEN[@]}"; do
    [ -e "$SRC/$s/$f" ] && die "$s/$f 是填好的真配置（不是 .example 模板）—— 先把它挪出 skill 目录再导出"
  done
  cp -R "$SRC/$s" "$STAGE/$s"
  for d in "${EXCLUDE_DIRS[@]}"; do rm -rf "${STAGE:?}/$s/$d"; done
  find "$STAGE/$s" -name .DS_Store -delete
done

# ── 2) 体检暂存树 ──────────────────────────────────────────────────────────
allowed() { # $1 = "相对路径:行号:命中文本" → 命中豁免表则 0
  local f="${1%%:*}" m="${1#*:}" e p re
  m="${m#*:}"
  for e in "${ALLOW[@]}"; do
    p="${e%%::*}"; re="${e#*::}"; re="${re%%::*}"
    [ "$p" = "*" ] || [ "$p" = "$f" ] || continue
    if printf '%s' "$m" | grep -qE -- "$re"; then
      allow_reason="${e##*::}"
      return 0
    fi
  done
  return 1
}

findings=0; allowed_n=0
for rule in "${RULES[@]}"; do
  label="${rule%%|*}"; re="${rule#*|}"
  while IFS= read -r hit; do
    [ -n "$hit" ] || continue
    if allowed "$hit"; then
      allowed_n=$((allowed_n + 1))
      info "豁免 [$label] ${hit}（${allow_reason}）"
      continue
    fi
    [ "$findings" = 0 ] && printf '%s体检不过 —— 这些东西不该出仓：%s\n' "$c_red" "$c_off"
    findings=$((findings + 1))
    printf '  %-14s %s\n' "[$label]" "$hit"
  done < <(cd "$STAGE" && grep -rIEno -- "$re" . 2>/dev/null | sed 's|^\./||' | sort -u)
done

if [ "$findings" -gt 0 ]; then
  echo
  echo "共 $findings 处。skill 路径应从本 skill 目录起算；环境信息改成占位（<registry> / <你的域名> 这类），"
  echo '或者确认它确实是夹具/占位之后，加进脚本里的 ALLOW 表（要写理由）。不导出任何东西。'
  exit 1
fi

printf '%s体检通过%s（%d 条规则；豁免 %d 处 —— 理由见脚本里的 ALLOW 表）\n' "$c_grn" "$c_off" "${#RULES[@]}" "$allowed_n"

[ "$CHECK_ONLY" = 1 ] && { info "--check：不写盘"; exit 0; }

# ── 3) 落盘 ────────────────────────────────────────────────────────────────
mkdir -p "$DST"
DST="$(cd "$DST" && pwd -P)"
[ "$DST" = "/" ] && die "目标目录不能是 /"
for family in .agents .claude; do
  case "$DST/" in
    "$ROOT/$family/skills/"*) die "目标目录不能位于本仓的 $family/skills 内，以免覆盖 skill 源文件" ;;
  esac
done
info "导出到 $DST"
for s in "${SKILLS[@]}"; do
  rm -rf "${DST:?}/$s"                       # 只删名单里的这几个目录，目标目录的其余内容不动
  cp -R "$STAGE/$s" "$DST/$s"
  printf '  %-22s %s 个文件\n' "$s" "$(find "$DST/$s" -type f | wc -l | tr -d ' ')"
done

if git -C "$DST" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo
  echo "目标仓的改动（未提交）："
  git -C "$DST" status --short -- "$DST" | sed 's/^/  /'
fi
