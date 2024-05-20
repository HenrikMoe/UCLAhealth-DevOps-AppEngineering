
###

# return resource name with proper transformation per the requirements

output "resource_name" {

  #tertiary transformations

  value = format(
    "%s%s%s",
    var.resource_type == "virtual_machine" ? "vm-" :
    var.resource_type == "key_vault" ? "kv-" :
    var.resource_type == "storage_account" ? "sa" : "",
    var.resource_type == "virtual_machine" ? local.truncated_base_name : lower(replace(var.base_name, "-", "")),
    var.resource_type == "virtual_machine" ? "-00" : ""
  )

}

#requirements
#If the resource_type is "virtual_machine" you should take the "base_name" and append "vm-" to the front of it and "-00" to the back. If the base name and and values you're appending are greater than 15 characters you should truncate the "base_name" only in order to be 15 characters when combined with your additional characters. Note that the base_name can be greater than 15 characters.
#If the resource_type is "key_vault" you should append "kv-" to the front of "base_name" and then set all characters to lower case.
#If the resource_type is "storage_account" you should append "sa" to the front of "base_name and then remove all "-" from the name and then set all characters to lower case.
