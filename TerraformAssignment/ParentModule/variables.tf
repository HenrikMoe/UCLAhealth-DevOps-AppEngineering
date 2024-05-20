
###

# declare a map type for passing input in main

variable "resource_map" {

  description = "Map of base names to resource types"
  type = map(string)

}
