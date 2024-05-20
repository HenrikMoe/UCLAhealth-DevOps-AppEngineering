
###

output "resource_names" {

  # extract keys and values from map and repack into child interface

  value = { for k, v in module.resource_names : k => v.resource_name }

}
