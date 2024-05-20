
###

# interface for the 15 character cap 

locals {
  truncated_base_name = substr(var.base_name, 0, 15 - length("-vm-00"))
}
