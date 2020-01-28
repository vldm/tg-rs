(function() {var implementors = {};
implementors["carapax_access"] = [{text:"impl&lt;C, P&gt; <a class=\"trait\" href=\"carapax/handler/trait.Handler.html\" title=\"trait carapax::handler::Handler\">Handler</a>&lt;C&gt; for <a class=\"struct\" href=\"carapax_access/struct.AccessHandler.html\" title=\"struct carapax_access::AccessHandler\">AccessHandler</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"carapax_access/trait.AccessPolicy.html\" title=\"trait carapax_access::AccessPolicy\">AccessPolicy</a>&lt;C&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>",synthetic:false,types:["carapax_access::handler::AccessHandler"]},];
implementors["carapax_ratelimit"] = [{text:"impl&lt;C&gt; <a class=\"trait\" href=\"carapax/handler/trait.Handler.html\" title=\"trait carapax::handler::Handler\">Handler</a>&lt;C&gt; for <a class=\"struct\" href=\"carapax_ratelimit/struct.DirectRateLimitHandler.html\" title=\"struct carapax_ratelimit::DirectRateLimitHandler\">DirectRateLimitHandler</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>",synthetic:false,types:["carapax_ratelimit::direct::DirectRateLimitHandler"]},{text:"impl&lt;C, K&gt; <a class=\"trait\" href=\"carapax/handler/trait.Handler.html\" title=\"trait carapax::handler::Handler\">Handler</a>&lt;C&gt; for <a class=\"struct\" href=\"carapax_ratelimit/struct.KeyedRateLimitHandler.html\" title=\"struct carapax_ratelimit::KeyedRateLimitHandler\">KeyedRateLimitHandler</a>&lt;K&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"carapax_ratelimit/trait.RateLimitKey.html\" title=\"trait carapax_ratelimit::RateLimitKey\">RateLimitKey</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>",synthetic:false,types:["carapax_ratelimit::keyed::KeyedRateLimitHandler"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()