# Security Documentation

## Overview

This document outlines the security measures implemented in the MCP Command Server and provides guidance for secure deployment and operation.

## Vulnerability History

### CVE-2024-XXXX: Whitespace Injection Bypass (FIXED)

**Severity**: Critical (CVSS 9.8)

**Status**: Fixed in version 0.1.1

**Reporter**: scott-joe via mcpscan.ai

#### Description

A critical security vulnerability was discovered where attackers could bypass the command denylist by using alternative whitespace characters (tabs, newlines, carriage returns) instead of spaces in dangerous commands.

#### Technical Details

- **Root Cause**: Parsing discrepancy between validator and executor
- **Validator**: Used `contains()` on raw command string
- **Executor**: Used `split_whitespace()` which normalizes all whitespace
- **Attack Vector**: `rm\t-rf` bypassed `rm -rf` filter but executed as `rm -rf`

#### Fix Implementation

The vulnerability was fixed by normalizing commands in the validator using the same parsing logic as the executor:

```rust
// BEFORE (vulnerable):
let command_to_check = command.to_lowercase();

// AFTER (secure):
let parts: Vec<&str> = command.split_whitespace().collect();
let canonical_command = parts.join(" ");
let command_to_check = canonical_command.to_lowercase();
```

#### Impact
- **Before Fix**: Attackers could execute arbitrary dangerous commands
- **After Fix**: All dangerous commands blocked regardless of whitespace used

## Current Security Measures

### 1. Command Validation System

#### Pattern-Based Filtering
- **Plain Text Patterns**: Exact substring matching against dangerous commands
- **Regular Expression Patterns**: Advanced pattern matching for complex threats
- **Case Insensitive**: Default behavior to catch variations
- **Canonical Normalization**: Commands normalized before validation

#### Blocked Command Categories
- System modification (`apt`, `yum`, `sudo`)
- File deletion (`rm -rf`, `shred`)
- System control (`shutdown`, `reboot`)
- Network operations (`wget`, `curl`, `ssh`)
- Command chaining (`&&`, `||`, `;`)
- Script execution (`bash`, `python`, `.sh`)
- Filesystem traversal (`../`, `~/`)

### 2. Security Logging

All blocked commands are logged with the following information:
- Original command as received
- Canonical normalized command
- Matching pattern or regex
- Timestamp and security alert level

Example log entry:
```
SECURITY ALERT: Command blocked by substring match - Original: 'rm\t-rf /tmp', Canonical: 'rm -rf /tmp', Pattern: 'rm -rf'
```

### 3. Container Security

#### Non-Root Execution
- Server runs as non-privileged user inside container
- Limits potential damage from successful attacks
- Follows principle of least privilege

#### Resource Isolation
- Containerized environment isolates server from host system
- Limited filesystem access
- Network restrictions can be applied via Docker configuration

### 4. Input Validation

#### Command Parsing
- Empty commands rejected
- Whitespace normalization prevents injection attacks
- Consistent parsing between validation and execution

#### JSON-RPC Validation
- Strict parameter validation
- Required fields enforced
- Malformed requests rejected

## Security Best Practices

### Deployment

1. **Container Security**
   ```bash
   # Run with minimal privileges
   docker run --user 1000:1000 --read-only mcp_command_server
   
   # Limit resources
   docker run --memory=256m --cpus=0.5 mcp_command_server
   
   # Network restrictions
   docker run --network=none mcp_command_server  # For isolated environments
   ```

2. **Firewall Configuration**
   - Restrict access to port 3030
   - Use allowlist of authorized IP addresses
   - Consider VPN or private network deployment

3. **Monitoring**
   - Monitor logs for security alerts
   - Set up alerting for blocked commands
   - Regular security audits

### Configuration

1. **Exclude Patterns**
   - Regularly update `exclude.yaml` with new threats
   - Test patterns thoroughly before deployment
   - Use both plain text and regex patterns for comprehensive coverage

2. **Options Configuration**
   ```yaml
   options:
     case_sensitive: false    # Recommended: false for broader protection
     whole_command: false     # Recommended: false for substring matching
     allow_override: false    # Recommended: false for strict security
   ```

### Monitoring and Alerting

1. **Log Monitoring**
   ```bash
   # Monitor for security alerts
   docker logs mcp_command_server 2>&1 | grep "SECURITY ALERT"
   
   # Set up log forwarding to SIEM
   docker run --log-driver=syslog mcp_command_server
   ```

2. **Metrics Collection**
   - Track blocked command attempts
   - Monitor unusual patterns
   - Alert on repeated violation attempts

## Threat Model

### Assets
- Host system integrity
- Data confidentiality
- Service availability

### Threats
1. **Command Injection**: Bypassing filters to execute dangerous commands
2. **Data Exfiltration**: Using allowed commands to extract sensitive data
3. **Denial of Service**: Resource exhaustion through command execution
4. **Privilege Escalation**: Attempting to gain higher privileges

### Mitigations
1. **Defense in Depth**: Multiple layers of security controls
2. **Principle of Least Privilege**: Minimal required permissions
3. **Input Validation**: Strict command filtering
4. **Monitoring**: Real-time threat detection

## Incident Response

### Security Alert Response
1. **Immediate**: Block suspicious IP addresses
2. **Short-term**: Review and update exclusion patterns
3. **Long-term**: Analyze attack patterns and improve defenses

### Vulnerability Disclosure
- Report security issues to: security@[domain]
- Use encrypted communication for sensitive reports
- Follow responsible disclosure timeline

## Security Testing

### Regular Testing
1. **Penetration Testing**: Regular security assessments
2. **Vulnerability Scanning**: Automated security scans
3. **Code Review**: Security-focused code reviews

### Test Cases
```bash
# Test whitespace injection prevention
curl -X POST -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "command/get",
  "params": {"command": "rm\t-rf /tmp"}
}' http://localhost:3030/

# Should return error: Command blocked
```

## Compliance

### Security Standards
- OWASP Top 10 compliance
- CIS Security Controls alignment
- Industry best practices implementation

### Audit Trail
- All security events logged
- Immutable log storage recommended
- Regular log review and analysis

## Updates and Maintenance

### Security Updates
- Monitor for new vulnerabilities
- Apply security patches promptly
- Test updates in staging environment

### Pattern Updates
- Regular review of exclusion patterns
- Community-driven threat intelligence
- Automated pattern updates where possible

---

**Last Updated**: 2025-09-24
**Version**: 1.0
**Next Review**: 2025-12-24
