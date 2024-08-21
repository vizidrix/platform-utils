
    /// Instantiate a new `State` for a given `Request`. This is primarily useful if you're calling
    /// Gotham from your own Hyper service.
    pub fn from_request(req: Request<Body>, client_addr: SocketAddr) -> Self {
        let mut state = Self::new();

        put_client_addr(&mut state, client_addr);

        let (
            request::Parts {
                method,
                uri,
                version,
                headers,
                mut extensions,
                ..
            },
            body,
        ) = req.into_parts();

        state.put(RequestPathSegments::new(uri.path()));
        state.put(method);
        state.put(uri);
        state.put(version);
        state.put(headers);
        state.put(body);

        if let Some(on_upgrade) = extensions.remove::<OnUpgrade>() {
            state.put(on_upgrade);
        }

        {
            let request_id = set_request_id(&mut state);
            debug!(
                "[DEBUG][{}][Thread][{:?}]",
                request_id,
                std::thread::current().id(),
            );
        };

        state
    }


    