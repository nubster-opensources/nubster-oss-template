# Security policy

## Supported versions

{{project-name}} follows the [semver policy](docs/SEMVER_POLICY.md). During the 0.x phase, only the latest minor release receives security fixes.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

The supported window will be widened once {{project-name}} reaches 1.0.

## Reporting a vulnerability

If you find a security vulnerability in {{project-name}}, please **do not** open a public GitHub issue. Disclosure rules:

1. Email a detailed report to **security@nubster.com** with the subject prefix `[{{project-name}} security]`.
2. The report should include:
   - A description of the vulnerability and the attacker model.
   - Affected versions and crates.
   - Reproduction steps or a proof of concept.
   - The impact you anticipate (data leak, denial of service, privilege escalation, etc.).
   - Suggested mitigation if you have one.
3. You will receive an acknowledgement within **7 calendar days**. If you do not, please follow up at the same address.
4. We will work with you to validate, scope and remediate the issue. A coordinated disclosure timeline will be agreed in writing. The default embargo period is **90 days** from acknowledgement.
5. Once a fix is published, you will be credited in the release notes unless you prefer to remain anonymous.

## Encrypted reporting

If you prefer encrypted communication, request a PGP key in your first email to security@nubster.com.

## Out of scope

The following are explicitly **out of scope** for vulnerability reports:

- Issues in unsupported versions.
- Vulnerabilities in third-party dependencies that are already publicly disclosed and tracked upstream. Report them to the upstream project.
- Reports based on theoretical attacks without a working proof of concept.
- Resource exhaustion caused by inputs that the library user deliberately constructs.

TODO: extend this section with a threat model specific to {{project-name}} as the project matures.

## Public security advisories

Security advisories are published as GitHub Security Advisories on the [repository security page](https://github.com/nubster-opensources/{{project-name}}/security/advisories) once the embargo period ends.
