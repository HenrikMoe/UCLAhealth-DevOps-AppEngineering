
###

# interface for base_name, validated through transformation in main.tf

variable "base_name" {

  description = "The base name of the resource"
  type = string

}

# local var interface for validating the value of the resource type from input

variable "resource_type" {

  description = "Resource type validation interface (virtual_machine, key_vault, storage_account)"
  type = string
  
  validation {
    condition = contains(["virtual_machine", "key_vault", "storage_account"], var.resource_type)
    error_message = "The resource_type must be either virtual_machine, key_vault, or storage_account."
  }

}
