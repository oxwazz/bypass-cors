# Contributing Guidelines

*Pull requests, bug reports, and all other forms of contribution are welcomed and highly encouraged!* :octocat:

> **This guide serves to set clear expectations for everyone involved with the project so that we can improve it
together while also creating a welcoming space for everyone to participate. Following these guidelines will help ensure
a positive experience for contributors and maintainers.**

### Contents

- [Install](#books-install)
- [How it Works](#thread-how-it-works)
- [Tools](#wrench-tools)

## :books: Install

Before you start contributing you must clone and installing this project on your local machine.

> Prerequisite: you must install [rust](https://www.rust-lang.org/tools/install) v1.56+
> and [node](https://nodejs.org/en/download/package-manager/current) v20+ on your machine first

1. Clone the project

```sh
# using ssh
git clone git@github.com:oxwazz/cors_bypass.git
# or using https
git clone https://github.com/oxwazz/cors_bypass.git
```

2. Open and run

```sh
# opening project
cd cors_bypass
# run
npx wrangler dev
```

done 🎉

## :thread: How it works

This code is deployed on [Cloudflare worker](https://developers.cloudflare.com/workers/languages/rust/). The entire script is on lib.rs.
It makes the request exactly as defined by the user, retrieves the response from the target server, and returns it with 
additional headers to ensure it is CORS-compliant.

## :wrench: Tools

While working this project I'm usually using this tools, like:

1. https://jsonplaceholder.typicode.com - dummy API
1. https://ngrok.com/ - expose local server
