# ruthenium

## Web

cargo install cargo-edit
cargo install cargo-watch

### cors

一步一步解决异步跨域请求 CORS（跨域资源共享）报错
https://devnote.pro/posts/10000003471151

## App

yarn global add typescript-language-server typescript

yarn create react-app app --template typescript

### dependencies

antd
qs
@types/qs
@emotion/react
@emotion/styled

### Antd design

[link](https://ant.design/docs/react/use-in-typescript-cn)

1. yarn add antd

2. import css
   vim src/App.css
   `@import '~antd/dist/antd.css';`

### datepicker locale

yarn add moment

```typescript
import 'moment/locale/zh-cn';
import locale from 'antd/es/date-picker/locale/zh_CN';
<DatePicker locale={locale} />;
```
