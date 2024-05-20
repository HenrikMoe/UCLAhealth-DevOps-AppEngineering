# Purpose and Output

Create resource names for a given set of rules.

# Configure

Open main.tf in the local directory and code an import of resources or hardcode like it is currently.

```
  resource_map = {
    "vm_1_config" = "virtual_machine"
    "hospital_System_A_Key" = "key_vault"
    "patient_record" = "storage_account"
  }
```

# Commands

`terraform init`

`terraform apply`

# Dev Path

expand configuration by initializing [resource] deployment terraform functionality
