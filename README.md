## Notetaking for the lazy.

I love having notes.
I hate taking notes.

This cli takes a prompt and generates a note with ChatGTP based on a `prompt`.   
A `url` can be included to be scraped, and the contnet will ba added as a refernece to a prompt.

The notes are stored in markdown format in the folder set with `notes_path`.

Example:
```
gptnotes --prompt "creating local cloudflare tunnel" --url "https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/tunnel-guide/local/#set-up-a-tunnel-locally-cli-setup" --category cloudflare
```

Will createa a file called `creating_local_cloudflare_tunnel.md` in the `notes_path` folder:
```
To create a local Cloudflare tunnel, follow these steps:
1. Download and install Cloudflared for your system.
2. Authenticate Cloudflared by running `cloudflared tunnel login`.
3. Create a tunnel by running `cloudflared tunnel create <NAME>`, which generates a tunnel credentials file and a subdomain of .cfargotunnel.com.
4. Create a configuration file with specific fields for connecting an application or network.
5. Assign a CNAME record that points traffic to your tunnel subdomain.
6. Run the tunnel by running `cloudflared tunnel run <UUID or NAME>`.
7. Check the tunnel configuration by running `cloudflared tunnel info <UUID or NAME>`.

Related URL: [To create a local Cloudflare tunnel, follow these steps:](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/tunnel-guide/local/#set-up-a-tunnel-locally-cli-setup)
```

I use this to create the base notes I then refine them in [Obsidian](https://obsidian.md/).
