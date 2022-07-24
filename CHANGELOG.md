# Changelog

## Unreleased (v0.1.0-beta.2)

### New changes
- @dsolartec `feat(schema): Expose collection name`
- @dsolartec `feat(error): Wrap  MongoDB document access error`
- @dsolartec `feat(schema): Add aggregate method (MongoDB aggregations)`
- @dsolartec `feat: update cargo edition to 2021`
- @dsolartec `feat: update mongodb crate version to 2.3.0`
- @dsolartec `feat(nongoose): add find_one_and_remove method`
- @dsolartec `feat(nongoose): add find_and_remove method`
- @dsolartec `feat(tests): add find and remove (one or more) tests`
- @dsolartec `feat(nongoose): add find_by_id_and_remove method`
- @dsolartec `feat(tests): add find by id and remove test`

### Bugfixes

- @dsolartec `fix(docs): Unicode bug and orthographic error`
- @dsolartec `fix(tests): Change 'async' feature to 'tokio-runtime'`
- @dsolartec `fix(nongoose): find_one_and_remove sync method`

## v0.1.0-beta.1

### New changes

- @dsolartec `feat: Add populate, sync functions and schema_relations macro attribute`
- @dsolartec `feat(mongodb): Move mongodb export to lib.rs`
- @dsolartec `feat(nongoose): Add find one document query`
- @dsolartec `feat(relations): Add one to many and optional id field attribute`
- @dsolartec `feat(schema): Add Into<Bson> implementation`
- @dsolartec `feat(schema): Add save function`
- @dsolartec `feat(nongoose): Change to save schema function in create function`
- @dsolartec `feat(schema): Add before functions`
- @dsolartec `feat(docs): Start documentation with mdbook`
- @dsolartec `feat(github): Add workflows`
- @dsolartec `feat(nongoose): Add find and find_one options`
- @dsolartec `feat(nongoose): Add update many`
- @dsolartec `feat(schema): Add remove function`
- @dsolartec `feat(nongoose): Add count function and tests`

### Bugfixes

- @dsolartec `fix(find_by_id): Change return from 'T' to 'Option<T>'`
- @dsolartec `fix(relations): Set relation value`
