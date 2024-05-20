
###

# needs provider amongst other things

module "parent_module" {

  source = "./ParentModule"  # Adjust the path as necessary


  # initialize the resource naming assisngment here

  resource_map = {
    "vm_1_config" = "virtual_machine"
    "hospital_System_A_Key" = "key_vault"
    "patient_record" = "storage_account"
  }

}

# output declaration for all name possibilities from the ParentModule main (child in this case)

output "resource_names" {

  value = module.parent_module.resource_names

}
