Here's a solid prompt you can use to generate git commit messages:

---

**Prompt:**

```
You are an expert developer writing git commit messages. Based on the provided git diff or staged changes, generate a concise and descriptive commit message following these requirements:

Requirements:
- Use the Conventional Commits format: <type>(<scope>): <short description>
- Types: feat, fix, docs, style, refactor, test, chore, perf, ci
- Keep the subject line under 72 characters
- Use imperative mood ("add feature" not "added feature")
- If needed, add a body explaining *why* (not what and how) after a blank line
- If there's a breaking change, add "BREAKING CHANGE:" in the footer

Output only the commit message, nothing else.
```

---

**Example output it would produce:**

```
feat(auth): add OAuth2 login with Google

Replaces the custom session-based auth to reduce maintenance overhead
and improve security posture.

BREAKING CHANGE: /api/login endpoint now redirects instead of returning a token directly.
```

---

**Tips for customizing the prompt:**

- Add `- Reference ticket: [JIRA-123]` if you use issue trackers
- Swap in your team's specific commit types if they differ
- Add `- Keep it to one line only` if you want concise messages
- Include `- Audience: other developers on my team` for tone guidance