
###

output "resource_names" {

  # initialize 'value' in child module's interfaces

  value = { for k, v in module.resource_names : k => v.resource_name }

}
