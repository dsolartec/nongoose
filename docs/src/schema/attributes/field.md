# Field Attributes

- `#[schema(id)]` _Required_

  Represents the id of the document (`_id` in MongoDB).

  _Note: The schema only supports one id field._

- `#[schema(unique)]`

  Unique this field: the field value cannot be duplicated in the document.

- `#[schema(convert = "path")]`

  Call a function to convert the field type to a BSON type.

- `#[schema(many_to_one = "Schema")]`

  Many to one relation.

- `#[schema(one_to_one = "Schema")]`

  One to one relation.

- `#[schema(one_to_many = "Schema")]`

  One to many relation.

- `#[schema(optional)]`

  Optional relation id(s) field(s).
