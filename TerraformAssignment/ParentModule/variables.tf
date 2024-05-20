
###

# declare a map type for passing input to the child module transformation interface

variable "resource_map" {

  description = "Map of base names to resource types"
  type = map(string)

}
