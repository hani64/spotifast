---
title: Make It Even Faster
description: "Reduce loading delays with your own Spotify connection."
nav_order: 6
---

## API rate limits

Spotify limits how often apps can ask for information. Spotifast normally
shares this allowance with other listeners and several other music players.
When that shared connection is busy, your library and search results can take
longer to load. The top bar shows a spinner while you wait.

You can reduce those delays by creating a **personal Spotify app**. This is
a connection registered to your account on Spotify's developer website.
You do not need to write code or install another player.

Spotifast offers this setup once after you sign in with Premium. Choose
**Set up personal app** to open Settings, or **Keep shared app** to continue
as you are. You can set it up later in Settings.

A personal app gives many of your searches and library requests a separate
allowance. Creating one is free and takes a few minutes, and Spotify requires
a Premium account. If you create several personal apps, they share your
account's allowance, following Spotify's
[July 2026 quota update](https://developer.spotify.com/blog/2026-07-23-web-api-quota-updates).
Your personal connection has limits too. If it cannot be restored, Spotifast
tries the saved shared connection.

## Personal and playback coverage

Your personal connection handles account and library requests, including
playlist search. Search results still appear as each part is ready. Spotify
may omit Spotify-owned playlists from a personal app's search results.

With local playback connected, Spotifast reads your complete playlist list,
artist popular tracks, and song radio through that session. It also uses the
session for the **Made for you** shelf and related artists. Home's suggested
songs use radio seeded by one top track; they may differ from Spotify's Web API
recommendations. These session features depend on Spotify's private responses,
which can change. If a session read fails, an unsupported feature may be
unavailable while the personal connection is active.

Spotify allows personal apps ten search results at a time for each type,
compared with twenty on the shared connection.

Setting up playback on this computer also helps playlists load faster through
its music connection.
[How it connects](/how-it-connects/) explains which connection each feature uses.

## Make a Spotify app

1. Open the [Spotify developer dashboard](https://developer.spotify.com/dashboard)
   and sign in with your Spotify account. Spotify asks that it be a
   Premium account.
2. Click **Create app**. Any name and description will do; nobody else
   sees them.
3. Under **Redirect URIs**, add exactly:

   ```
   http://127.0.0.1:8989/login
   ```

4. Tick **Web API**, accept the terms, and save.
5. The app's page shows its **Client ID**. Copy it.

![Settings, with a personal Spotify app in use](/assets/images/make-it-even-faster.png)

## Use it in Spotifast

1. Open **Settings**, find **Personal Spotify app**, and paste the
   Client ID.
2. Click **Authorize**. Your browser opens Spotify's sign-in for your app.
   Spotifast verifies that it belongs to the same Spotify account, then shows
   **Personal app ready**.

Your playback setup stays the same. Select **Remove** to stop using your
personal connection and return to shared access.
