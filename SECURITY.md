# Security Policy

## Supported Versions

This repository is published for research and learning purposes. It is **not** a production-grade project and does not have a formal long-term support (LTS) schedule. That said, critical security reports will be reviewed and addressed when possible.

| Version / Branch | Supported |
| ---------------- | --------- |
| `main`           | :white_check_mark: |
| tagged releases  | :x: |
| older branches   | :x: |

> If you require a production-ready, actively maintained security posture, please consider using a mature/security-focused project or contact the maintainers before relying on this code in production.

---

## Reporting a Vulnerability

If you discover a security vulnerability, please report it privately so we can investigate and, where appropriate, produce a coordinated fix.

**Preferred reporting channels (in order):**
1. **GitHub Security Advisories** — create a private security advisory for this repository.  
2. **Email** — send details to **tnalsdk0914@gmail.com** (PGP not currently published).

### What to include in a report
Please include as much of the following information as you can to help us triage quickly:

- A short, descriptive title.
- A clear description of the vulnerability and the affected component(s).
- Step-by-step reproduction instructions or a minimal PoC (proof of concept) that demonstrates the issue.
- The expected behavior vs. the observed behavior.
- The environment(s) where the issue was observed (OS, architecture, Rust version, crate versions, etc.).
- Any suggested fixes or mitigations (optional).
- Contact information so we can follow up (email or GitHub handle).

**Do not** post vulnerabilities in public issues or pull requests — this may expose users to risk before a fix is available.

---

## Response Process & Timeline

- **Acknowledgement:** You can expect an initial acknowledgement within **24–72 hours** of receipt.  
- **Triage:** We will assess severity and impact and provide interim guidance (e.g., mitigation steps) as needed.  
- **Fix & Disclosure:** If the report is confirmed, we will create a fix (or mitigation) and coordinate disclosure. We aim to publish a public disclosure only after a patch or mitigation is available.  
- **If we determine the report is not a vulnerability** (false positive or out of scope), we will explain our reasoning and close the report.

---

## Confidentiality & Credit

- We will treat reports confidentially while investigating.  
- If you would like credit in a public advisory or the changelog, please state that preference in your report. We will credit discoverers unless they request anonymity.

---

## Important Notes

- This repository is primarily for research and educational use. It may contain experimental code, unsafe examples, or intentionally insecure samples for learning — **do not** run in production without a security review.  
- If you require an encrypted communication channel (PGP), respond to the initial acknowledgment with that request and we will arrange it if possible.

Thank you for helping improve the security of this project.
