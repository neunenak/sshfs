use libc::{c_char, c_int};
use std::ffi::CStr;

pub fn is_ssh_opt(s: *const c_char) -> c_int {
    let c_string = unsafe { CStr::from_ptr(s) };

    let c_string_bytes = c_string.to_bytes();

    for opt in SSH_OPTS.iter() {
        if c_string_bytes == opt.as_bytes() {
            return 1;
        }
    }
    return 0;
}

pub fn is_ssh_opt_str(s: &str) -> bool {
    for opt in SSH_OPTS.iter() {
        if s == *opt {
            return true;
        }
    }
    false
}

const SSH_OPTS: [&'static str; 61] = [
    "AddressFamily",
    "BatchMode",
    "BindAddress",
    "BindInterface",
    "CertificateFile",
    "ChallengeResponseAuthentication",
    "CheckHostIP",
    "Cipher",
    "Ciphers",
    "Compression",
    "CompressionLevel",
    "ConnectionAttempts",
    "ConnectTimeout",
    "ControlMaster",
    "ControlPath",
    "ControlPersist",
    "FingerprintHash",
    "GlobalKnownHostsFile",
    "GSSAPIAuthentication",
    "GSSAPIDelegateCredentials",
    "HostbasedAuthentication",
    "HostbasedKeyTypes",
    "HostKeyAlgorithms",
    "HostKeyAlias",
    "HostName",
    "IdentitiesOnly",
    "IdentityFile",
    "IdentityAgent",
    "IPQoS",
    "KbdInteractiveAuthentication",
    "KbdInteractiveDevices",
    "KexAlgorithms",
    "LocalCommand",
    "LogLevel",
    "MACs",
    "NoHostAuthenticationForLocalhost",
    "NumberOfPasswordPrompts",
    "PasswordAuthentication",
    "PermitLocalCommand",
    "PKCS11Provider",
    "Port",
    "PreferredAuthentications",
    "ProxyCommand",
    "ProxyJump",
    "ProxyUseFdpass",
    "PubkeyAcceptedKeyTypes",
    "PubkeyAuthentication",
    "RekeyLimit",
    "RevokedHostKeys",
    "RhostsRSAAuthentication",
    "RSAAuthentication",
    "ServerAliveCountMax",
    "ServerAliveInterval",
    "SmartcardDevice",
    "StrictHostKeyChecking",
    "TCPKeepAlive",
    "UpdateHostKeys",
    "UsePrivilegedPort",
    "UserKnownHostsFile",
    "VerifyHostKeyDNS",
    "VisualHostKey",
];
