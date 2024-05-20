
###

# needs provider amongst other things

module "parent_module" {

  # parent module import 

  source = "./ParentModule"  


  # reconfigure the resource naming assignments here

  resource_map = {
    "vm_1_config" = "virtual_machine"
    "hospital_System_A_Key" = "key_vault"
    "patient_record" = "storage_account"
  }

}

# output the layered declaration for all name possibilities from the ParentModule (child in this case)

output "resource_names" {

  value = module.parent_module.resource_names

}
