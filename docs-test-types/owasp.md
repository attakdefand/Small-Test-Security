api-test/
└── tests/
    └── owasp/
        ├── common/
        │   └── mod.rs              # tiny HTTP client + base URL helper
        ├── top10/
        │   ├── a01_broken_access_control.rs
        │   ├── a02_cryptographic_failures.rs
        │   ├── a03_injection.rs
        │   ├── a04_insecure_design.rs
        │   ├── a05_security_misconfiguration.rs
        │   ├── a06_vulnerable_outdated_components.rs
        │   ├── a07_identification_auth_failures.rs
        │   ├── a08_software_data_integrity_failures.rs
        │   ├── a09_logging_monitoring_failures.rs
        │   └── a10_ssrf.rs
        ├── api_top10/
        │   ├── api1_bola.rs
        │   ├── api2_broken_auth.rs
        │   ├── api3_property_level_auth.rs
        │   ├── api4_unrestricted_resource_consumption.rs
        │   ├── api5_function_level_auth.rs
        │   ├── api6_sensitive_business_flows.rs
        │   ├── api7_ssrf.rs
        │   ├── api8_security_misconfiguration.rs
        │   ├── api9_improper_inventory.rs
        │   └── api10_unsafe_api_consumption.rs
        └── asvs/
            └── README.md
