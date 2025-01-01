# Cors Bypass

[badge-workflow]: https://img.shields.io/github/actions/workflow/status/oxwazz/cors_bypass/release.yml
[link-workflow]: https://github.com/oxwazz/cors_bypass/actions/workflows/release.yml
[badge-twitter]: https://img.shields.io/twitter/follow/oxwazz
[link-twitter]: https://x.com/oxwazz

[![badge-workflow]][link-workflow]
[![badge-twitter]][link-twitter]

📷 Effortlessly access to your backend APIs and say goodbye to CORS headaches!

> [!WARNING]
> This API is intended for development purposes only and should not be used in production environments.

### Contents

- [Usage](#usage)
- [FAQ](#faq)
- [Why](#why)
- [Limitation](#limitation)
- [Contributing](#contributing)
- [Credit](#credit)
- [License](#license)

## Usage

🎩 If you want to access endpoint that has cors restriction for example `https://cors-restriction.com/users?user_id=1`,
simply prepend it with `https://cors-bypass.oxwazz.com/` to make it work seamlessly. 

like this:
```
https://cors-bypass.oxwazz.com/https://cors-restriction.com/users?user_id=1
```

Congratulation! 🎉 You are now bypassing the endpoint that has cors restriction!

## FAQ

**1. Why does text/html convert to text/plain, like when accessing facebook.com?**

> We convert text/html responses to text/plain to address security concerns. If we return text/html directly, 
> many browsers might label our domain as a potentially dangerous site due to the risk of malicious content being 
> executed. This precaution helps protect users and ensures the safety of the platform while preventing unnecessary 
> browser warnings.

## Why

🖌️ We created this project to help developers access their backend APIs without worrying about CORS restrictions. 
We understand how challenging it can be to juggle development tasks while staying productive, 
so this tool is designed to make your life easier and let you focus on what matters most—building great applications.

> [!WARNING]
> Use this for development purposes only and should not be used in production environments.

## Limitation

📐 This code is deployed on a [Cloudflare worker](https://developers.cloudflare.com/workers/languages/rust/). As we are
utilizing the free tier, there are [certain limitations](https://developers.cloudflare.com/workers/platform/limits/#worker-limits)
associated with it.

## Contributing

🎈 Thanks for your help improving the project! We are so happy to have you! We have a
[contributing guide](./CONTRIBUTING.md) to help you get involved in the project.

## Credit

📌 Cors Bypass is currently being developed and maintained by [Muhammad Rahmahalim](https://github.com/oxwazz).<br>
Thank you!

## License

[MIT](./LICENSE) License © 2024 [Muhammad Rahmahalim](https://github.com/oxwazz)