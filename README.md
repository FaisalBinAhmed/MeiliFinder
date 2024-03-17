# MeiliFinder ʕʘ̅͜ʘ̅ʔ 

## The ultimate MeiliSearch client for your terminal!

MeiliFinder is a beautiful and fast terminal-based MeiliSearch client built with Rust 🦀 and Ratatui. 

### Why MeiliFinder?

MeiliSearch is great; but there is no official client available for it. While there are some community-built clients, they are mostly web-based and lack important features. If you are a terminal lover like me or hate mouse, you may want to use a terminal-based client to interact with MeiliSearch. That's where MeiliFinder comes in. It is fast, robust, lightweight, and easy to use. It is also open-source, so you can contribute to the project and make it better.

### Features

- [x] Search for documents
- [x] Search for documents with sort and filters queries
- [ ] Search for documents with pagination
- [x] Delete individual documents
- [x] Bulk delete documents by filter
- [x] Connect to multiple MeiliSearch instances
- [ ] edit documents
- [ ] update index settings
- [x] Switch between indexes and instances on the fly
- [x] Tasks preview
- [ ] settings.json file for customizing the app
- [x] Toast notifications


## Using

Clone the repository and run `cargo run` in the root directory.
In case you don't have an `instances.json` file in the root directoty, it will prompt you to enter the MeiliSearch instance URL and the API key. Once you enter the details, the app will save the info in a new `instances.json` file, and you can start searching for documents. 

To exit the app, press `q` or `Ctrl+C`.


## Screenshots

![Documents Tab](https://imgur.com/Wx66b36)
![Document search](https://imgur.com/m3U5vP3)
![Document preview](https://imgur.com/BIXyxJZ)
![Bulk Delete action](https://imgur.com/CglAOhv)
![Indices](https://imgur.com/FvA9Q3m)
![Tasks](https://imgur.com/ohVbWsw)


## Shortcuts

### Normal mode

- `tab` - Switch between different tabs in the app.
- `r` - Refresh current list.
- `s` - Go to Search mode (insert search queries).
- `Up/Down` - Navigate through the current list.
- `Enter` - Select the current index/instance.
- `Space` - Quick preview of the highlighted document.
- `Ctrl+u` - Reset all search queries.
- `q` - Quit the app.
- `Ctrl+C` - Quit the app.

### Search mode

- `Esc` - Exit search mode.
- `tab` - Navigate through search/filter/sort queries.
- `Enter` - Perform search.

### Preview mode

- `Esc` - Exit preview mode.
- `backspace` - Go to Delete mode.

### Delete mode

- `Esc` - Exit delete mode.
- `backspace` - Confirm delete.


### instances.json file (optional)

The `instances.json` file is a JSON file that stores the MeiliSearch instance URL and the API key. The app uses this file to connect to the MeiliSearch instance. You can add multiple instances to the file and switch between them on the fly. 

The file should look like this:

```json
[
	{
		"id": "1",
		"name": "Your MeiliSearch Instance Name",
		"host": "https://your-meilisearch-instance.com",
		"primary_key": "your-api-key"
	}
]
```

Please refer to the sample `instances.json` file in the root directory.

## Installation Globally

To run it globally, you can install the app with `cargo binstall MeiliFinder`. Make sure you have `binstall` [binstall repo](https://github.com/cargo-bins/cargo-binstall) installed. Once installed, you can invoke the app just by running `MeiliFinder` in the terminal.
This is a binary crate and not a library, so you shouldn't use it as a dependency.

I might provide some pre-built binaries for Windows/MacOS/Linux in the future, or publish it on Homebrew / Winget.


## Credits

- MeiliSearch for providing the Rust SDK.
- Ratatui for the beautiful TUI framework.

## License

MIT

### Limitations

Currently, the app only handles ASCII or 1 byte UTF-8 character input. If you are searching a station with non-ASCII characters (i.e. "ö", "ß" etc.) in its name, the app will ignore the input. Please type the closest characters & scroll a bit down to select the station from the list (this will be fixed at a later version.)

