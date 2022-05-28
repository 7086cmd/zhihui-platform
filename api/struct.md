# The struct of the api

## Folder

The folder means `/` in the path, which can be used to route different part of the api.

For example:

```http
GET /api/v1/user/
```

This example reflects the api of getting user, and the desc file located:

```path
api/v1/user/get.yaml
```

There, if you want to get the deduction of all, you can use the api with following:

```http
GET /api/v1/deduction
```

which is located in:

```path
api/v1/deduction/get.yaml
```

## Inline Parameter

The inline parameter can be used to pass the parameter in the url, for e.g.

```http
GET /api/v1/user/1
```

This is following the api:

```http
GET /api/v1/user/:id
```

This is located in:

```path
/api/v1/user/param_id/get.yaml
```

So, undoubtedly, the inline parameter can be used to describe in `param_...` in the path.

## Methods

Like the `Folder` e.g., if you want to add a user, you can use the following url:

```http
POST /api/v1/user/
```

which is located:

```path
api/v1/user/post.yaml
```
