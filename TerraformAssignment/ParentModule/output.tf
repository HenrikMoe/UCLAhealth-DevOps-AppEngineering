
###

output "resource_names" {

  # extract keys and values from main for interfacing to child module 

  value = { for k, v in module.resource_names : k => v.resource_name }

}
