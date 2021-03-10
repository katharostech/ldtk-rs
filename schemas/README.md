# Schemas

These are the local schemas that we have pulled from the LDtk [git repo](https://github.com/deepnight/ldtk/tree/master/docs) so that internet access is not required to generate the schema.

## Patched Schemas

In LDtk v0.8.1 the JSON schema had the `__tileSrcRect` field marked as non-nullable when it happened to be null on the `Entities.ldtk` sample map, causing the map to fail to open. To remedy this we modified the schema to make the field nullable. Other than that change it is identical to the original v0.8.1 JSON schema.
