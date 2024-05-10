Hi there, this is a toy project to experiment building a simple blog _CRUD_ application with Rust, Axum and MongoDB.
This project provides a simple API for adding users and articles, and view/read all the articles as well
The goal of this project is **completely educational**, feel free to use it however you want.

🚨 It's still a work in progress. 🤓 💻

<div style="display:flex;">
  <img src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fblog.mbedded.ninja%2Fimages%2F2015%2F05%2Frust-programming-language-logo-white-background.png&f=1&nofb=1&ipt=0543185e400549a77f23b03b8170fc32bacd853b99bbc46c4b6f297e0d52e8ff&ipo=images" alt="Image 1" style="width:420;height:420;margin-right:10px;">
  <img src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.ictdemy.com%2Fimages%2F5728%2Fmdb.png&f=1&nofb=1&ipt=94dca8c3b0b2790aaef41ffd02a03ca2c98625d2ac787e62f79f4057ea41af96&ipo=images" alt="Image 2" style="width:420;height:420">
</div>

### I recommend using docker and docker compose to try the project, but feel free to use the other method as well.


# Feature List
* &#x2705; Add Article
* &#x2705; Add User
* &#x2705; View Articles
* &#x23f3; Delete Article
* &#x23f3; Authentication

# Usage

## Docker
You need to have docker and docker comopse installed.
* Install docker [Official Documentation](https://docs.docker.com/desktop/)
* Install docker compose [Official Documentation](https://docs.docker.com/compose/install/)
* After installation, run the following command to start the server and the database containers
    ### `$ docker compose up --build`
* Test the server on http://localhost:3000/

## Without Docker
You need to have rust and MongoDB installed.
* Run the following command to install Rust:
    ### `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
> NOTE: If you find the command sketchy, please head to the [official website](https://www.rust-lang.org/learn/get-started) and check for yourself 😏
* Install MongoDB (Community Edition) [Official Documentation](https://www.mongodb.com/docs/manual/administration/install-community/)
* Check that the mongodb server is running locally, for example, on Mac OS run:
    ### `brew services list | grep mongodb`
* If the server is in the _started_ status then everything should be fine.

> NOTE: Check the docs for linux and windows.
* Head to the project directory and run the web server
    ### `cargo run`


# Examples
## Add an article using curl
```
curl -X POST -H "Content-Type: application/json" -d '{
    "title": "Title goes here", \
    "body": "article body goes here" \
}' http://localhost:3000/api/article`
```

## View all existing articles
```
curl http://localhost:3000/api/article
```
