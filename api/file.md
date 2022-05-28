# File expression of the api desc

The file expression of the api desc is using `yaml`.

There are some parts of the expression file.

## Name

> types:
> `String` in Rust
> `string` in TypeScript

The `name` parameter is used to describe the name of the api.

e.g.

```yaml
name: Get User
```

## Path

> types:
> `String` in Rust
> `string` in TypeScript

The `path` parameter is used to describe the path of the api.

e.g.

```yaml
path: /api/v1/user/
```

## Method

> types:
> `String` in Rust
> `GET` | `POST` | `PUT` | `PATCH` | `DELETE` in TypeScript

The `method` parameter is used to describe the method of the api.

e.g.

```yaml
method: GET
```

## Description

the `description` parameter is used to describe the description of the api.
e.g.

```yaml
description: Get the user information.
```

## Data / Parameter

The `data` parameter is used to describe the data of the api, which is using the method of `POST`, `PUT`, `PATCH`, or `DELETE`.

The `param` parameter is used to describe the parameter of the api, which is using the method of `GET`.

### Name (In Data / Parameter)

The `name` parameter is used to describe the name of the parameter.

for e.g.

```yaml
name: id
```

### Type (In Data)

> This used `typescript` expression.

the `type` parameter is used to describe the type of the parameter.

e.g.

```yaml
type: string
```

### Required (In Data / Parameter)

the `required` parameter is used to describe the required of the parameter.

e.g.

```yaml
required: true
```

### In (In Data / Parameter)

the `in` parameter is used to describe the location of the parameter.

there is two locations: `query`, `path`, and `body`.

e.g.

```yaml
in: query
```

### Description (In Data / Parameter)

the `description` parameter is used to describe the description of the parameter.

e.g.

```yaml
description: The id of the user.
```

so, for e.g.

```yaml
data:
  - name: id
    type: string
    description: The id of the user.
```

Also, the `parameter` in the method of `GET` is same, which must use `string` as the type.

for e.g.

```yaml
params:
  - name: id
    description: The id of the user.
```

## Response

The `response` parameter is used to describe the response of the api.

It is same as the `data` parameter.

## Authorization

the `authorization` parameter is used to describe whether the api need authorizations.

e.g.

```yaml
authorization: true
```

so, a complete api desc file like:

```yaml
name: Create User
path: /api/v1/user/
method: POST
description: Create the user.
data:
  - name: id
    type: string
    required: true
    in: body
    description: The id of the user.
response:
  - name: id
    type: string
    description: The id of the user.
authorization: true
```
