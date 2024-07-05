# rust crate for [OpenAlex](https://docs.openalex.org/)
[![Build](https://github.com/0x6e66/openalex/actions/workflows/rust.yml/badge.svg)](https://github.com/0x6e66/openalex/actions/workflows/rust.yml)
[![Crate](https://img.shields.io/crates/v/openalex.svg)](https://crates.io/crates/openalex)
[![Documentation](https://img.shields.io/docsrs/openalex?label=docs.rs)](https://docs.rs/openalex)
![License](https://img.shields.io/crates/l/openalex)

## Description
This crate aims to provide interoperability between rust and the open publication database [OpenAlex](https://docs.openalex.org/). Currently these [OpenAlex API entities](https://docs.openalex.org/api-entities/entities-overview) are supported:
- [Works](https://docs.openalex.org/api-entities/works) ([here](./src/api_entities/work.rs))
- [Authors](https://docs.openalex.org/api-entities/authors) ([here](./src/api_entities/author.rs))
- [Sources](https://docs.openalex.org/api-entities/sources) ([here](./src/api_entities/source.rs))
- [Institutions](https://docs.openalex.org/api-entities/institutions) ([here](./src/api_entities/institution.rs))
- [Keywords](https://docs.openalex.org/api-entities/keywords) ([here](./src/api_entities/keyword.rs))
- [Publishers](https://docs.openalex.org/api-entities/publishers) ([here](./src/api_entities/publisher.rs))