# Changelog

## Unreleased (v0.1.0-beta.2)

### New changes
- @danielsolartech `feat(schema): Expose collection name`
- @danielsolartech `feat(error): Wrap  MongoDB document access error`
- @danielsolartech `feat(schema): Add aggregate method (MongoDB aggregations)`

### Bugfixes

- @danielsolartech `fix(docs): Unicode bug and orthographic error`
- @danielsolartech `fix(tests): Change 'async' feature to 'tokio-runtime'`

## v0.1.0-beta.1

### New changes

- @danielsolartech `feat: Add populate, sync functions and schema_relations macro attribute`
- @danielsolartech `feat(mongodb): Move mongodb export to lib.rs`
- @danielsolartech `feat(nongoose): Add find one document query`
- @danielsolartech `feat(relations): Add one to many and optional id field attribute`
- @danielsolartech `feat(schema): Add Into<Bson> implementation`
- @danielsolartech `feat(schema): Add save function`
- @danielsolartech `feat(nongoose): Change to save schema function in create function`
- @danielsolartech `feat(schema): Add before functions`
- @danielsolartech `feat(docs): Start documentation with mdbook`
- @danielsolartech `feat(github): Add workflows`
- @danielsolartech `feat(nongoose): Add find and find_one options`
- @danielsolartech `feat(nongoose): Add update many`
- @danielsolartech `feat(schema): Add remove function`
- @danielsolartech `feat(nongoose): Add count function and tests`

### Bugfixes

- @danielsolartech `fix(find_by_id): Change return from 'T' to 'Option<T>'`
- @danielsolartech `fix(relations): Set relation value`
