# Terraform Interview Assignment (5/20/2024)

Create resource names given a set of rules, maximizing modularity and code clarity for production.

Prompt: https://uclahealthcloud.notion.site/Senior-Cloud-DevOps-Engineer-flex-hybrid-Terraform-Prompt-47d1bf8c25ac436aba283d0260f2698c

## Getting Started 

`git clone https://github.com/HenrikMoe/UCLAhealth-DevOps-AppEngineering.git`

`cd TerraformAssignment`

## Configure Inputs

Open main.tf in the root of the ./TerraformAssignment directory. Change the code to import resources or change the hardcoded key/value pairs: 

```
  resource_map = {
    "vm_1_config" = "virtual_machine"
    "hospital_System_A_Key" = "key_vault"
    "patient_record" = "storage_account"
  }

```

## Commands

Install terraform (Mac OS with homebrew package manager) 

`brew install terraform`

Initialize the terraform configuration file set.

`terraform init`

Apply code changes to the terraform configuration.

`terraform apply`

## Development Path

Expand configuration by initializing [resource] deployment terraform functionality. Routing. Env secrets. 




