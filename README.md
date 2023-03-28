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
# Summary: Creating a Local Cloudflare Tunnel

To create a local Cloudflare tunnel, follow these steps:

1. Download and install Cloudflared for your OS by visiting the downloads page on Cloudflare's website.
2. Authenticate cloudflared by running `cloudflared tunnel login` in the terminal.
3. Create a tunnel and name it by running `cloudflared tunnel create <NAME>`.
4. Create a configuration file in your `~/.cloudflared` directory and add necessary fields.
5. Assign a CNAME record to your tunnel subdomain by running `cloudflared tunnel route dns <UUID or NAME> <hostname>`.
6. Run the tunnel by running `cloudflared tunnel run <UUID or NAME>`.
7. Check information on the tunnel you just created by running `cloudflared tunnel info <UUID or NAME>`. 

Note that you must add a website to Cloudflare and change your domain nameservers to Cloudflare before starting. Additionally, you may need to install cloudflared via your preferred package manager or build it from source. Finally, Cloudflare Tunnel can be installed as a system service or launch agent.

[reference](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/tunnel-guide/local/#set-up-a-tunnel-locally-cli-setup)
```

I use this to create the base notes and then refine them in [Obsidian](https://obsidian.md/).
