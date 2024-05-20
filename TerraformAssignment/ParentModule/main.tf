
###

module "resource_names" {

  # imports the naming transformation module

  source = "../ResourceNamingModule"

  # variables.tf map interface call, extracting the input into key val pairs with for_each tf functionality 

  for_each = var.resource_map

  # extract key val pairs to names for use in child module

  base_name = each.key
  resource_type = each.value

}
