# rust crate for [OpenAlex](https://docs.openalex.org/)
[![Build](https://github.com/0x6e66/openalex/actions/workflows/rust.yml/badge.svg)](https://github.com/0x6e66/openalex/actions/workflows/rust.yml)
[![Crate](https://img.shields.io/crates/v/openalex.svg)](https://crates.io/crates/openalex)
[![Documentation](https://img.shields.io/docsrs/openalex?label=docs.rs)](https://docs.rs/openalex)
![License](https://img.shields.io/crates/l/openalex)

## Description
This crate aims to provide interoperability between rust and the open publication database [OpenAlex](https://docs.openalex.org/). Currently these [OpenAlex API entities](https://docs.openalex.org/api-entities/entities-overview) are supported:
- [Works](https://docs.openalex.org/api-entities/works) ([here](./src/types/work.rs))
- [AUthors](https://docs.openalex.org/api-entities/authors) ([here](./src/types/author.rs))