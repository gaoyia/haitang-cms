# 管理接口

管理接口需要 JWT Token 授权。请在请求头中携带：

```
Authorization: Bearer <token>
```

多语言数据模型见 [多语言数据模型](../i18n-data-model.md)。读接口支持 `?lang=`；写接口可通过 body 中 `lang` 指定更新的语言行。
