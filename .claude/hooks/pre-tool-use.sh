#!/usr/bin/env bashh
#
# PreToolUse hook — MyHouse
# Exit 2 = bloque le tool call, stderr renvoyé à Claude comme feedback.
# Exit 0 = laisse passer.

set -euo pipefail

INPUT=$(cat)

TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name // empty')

# Ne traite que les appels Bash — tout le reste passe.
if [ "$TOOL_NAME" != "Bash" ]; then
    exit 0
fi

COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
    exit 0
fi

deny() {
    echo "BLOCKED by PreToolUse hook: $1" >&2
    exit 2
}

# ── 1. SQL destructif (psql / commandes shell contenant du SQL brut) ──────
# DROP / TRUNCATE : toujours bloqués, quel que soit le contexte.
if echo "$COMMAND" | grep -qiE '\b(DROP\s+(TABLE|DATABASE|SCHEMA)|TRUNCATE)\b'; then
    deny "SQL destructif détecté (DROP/TRUNCATE). Si intentionnel, exécute-le manuellement hors de l'agent."
fi

# DELETE FROM sans WHERE : bloqué. DELETE FROM ... WHERE ... passe.
if echo "$COMMAND" | grep -qiE '\bDELETE\s+FROM\b' \
   && ! echo "$COMMAND" | grep -qiE '\bWHERE\b'; then
    deny "DELETE FROM sans clause WHERE détecté. Ajoute une clause WHERE ou exécute manuellement."
fi

# ── 2. git commit / git push — toujours manuel, jamais par l'agent ────────
if echo "$COMMAND" | grep -qiE '(^|[;&|]\s*)git\s+commit\b'; then
    deny "git commit interdit à l'agent — commits effectués manuellement par l'utilisateur."
fi

if echo "$COMMAND" | grep -qiE '(^|[;&|]\s*)git\s+push\b'; then
    deny "git push interdit à l'agent — push effectués manuellement par l'utilisateur."
fi

# ── 3. Commandes git destructives (perte de travail non commité) ─────────
if echo "$COMMAND" | grep -qiE '(^|[;&|]\s*)git\s+reset\s+--hard\b'; then
    deny "git reset --hard interdit à l'agent — risque de perte de travail non commité."
fi

if echo "$COMMAND" | grep -qiE '(^|[;&|]\s*)git\s+checkout\s+--\s+\.'; then
    deny "git checkout -- . interdit à l'agent — écrase les modifications locales non commitées."
fi

if echo "$COMMAND" | grep -qiE '(^|[;&|]\s*)git\s+clean\s+-[a-z]*f'; then
    deny "git clean -f interdit à l'agent — suppression irréversible de fichiers non trackés."
fi

# ── 4. Protection .env (jamais lu, jamais staged, jamais catté) ──────────
# .env.example reste autorisé.
if echo "$COMMAND" | grep -qiE '(^|[;&|]\s*)git\s+add\s+.*(^|[/\s])\.env($|\s)'; then
    deny "git add .env interdit — fichier contenant des secrets, ne doit jamais être staged."
fi

if echo "$COMMAND" | grep -qiE '\b(cat|less|more|head|tail)\s+.*(^|[/\s])\.env($|\s)' \
   && ! echo "$COMMAND" | grep -qiE '\.env\.example\b'; then
    deny "Lecture de .env interdite à l'agent — fichier contenant des secrets."
fi

exit 0