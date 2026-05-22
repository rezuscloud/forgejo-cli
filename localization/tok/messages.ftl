msg-whoami = sina kepeken sijelo { $name }@{ $host }
msg-auth-login-oauth_unsupported =
    ilo FJ sina li ken ala kepeken nasin `login` lon ma { $host_domain }

    o lukin e lipu { $applications_url }
    o pali e nimi len ilo, o pana e ona tawa mi kepeken `fj auth add-key`
msg-auth-login-canceled = sina kama ala lon sijelo
msg-auth-login-browser_success = sina kama lon sijelo! o weka lipu ni, o tawa ilo toki sina
msg-auth-login-browser_failure = pakala li lon la sina ken ala kama lon sijelo
msg-auth_logout-success = sina weka tan sijelo { $username }@{ $host }
msg-auth_logout-already_signed_out = sina kepeken ala sijelo pi ma { $host }
msg-auth-use_ssh-not-logged-in = sina kepeken ala sijelo pi ma { $host } la mi ken ala ni
msg-auth-use_ssh-enabled = ma { $host } la mi kama jo e toki ilo la mi kama kepeken nasin SSH
msg-auth-use_ssh-disabled = ma { $host } la mi kama jo e toki ilo la mi kama kepeken ala nasin SSH
msg-auth-use_ssh-already_enabled = ma { $host } la mi kama jo e toki ilo la mi awen kepeken nasin SSH
msg-auth-use_ssh-already_disabled = ma { $host } la mi kama jo e toki ilo la mi awen kepeken ala nasin SSH
msg-auth-add_key-prompt = nimi len ilo sin:
msg-auth-add_key-already_exists = mi ken ala ni. nimi len ilo pi ma { $host } li lon
msg-auth-list-none = sina kepeken sijelo ala
msg-actions-variable-create-already_exists = sona ni li lon. sina wile ante e ona la, o kepeken nimi `--force`
msg-actions-variable-create-already_exists_forced = sona ni li lon. mi ante e ona
msg-actions-variable-delete-success = sona { $name } li weka
msg-org-list-no_results = alasa ni la kulupu ala li lon
msg-org-list-page_number = ni li kulupu nanpa { $page }. kulupu { $total } li lon
msg-org-view-org_name =
    { OPT($full_name) ->
       *[none] { STYLE("bold", "bright-cyan") }kulupu { $name }{ STYLE("reset") }
        [some] { STYLE("bold", "bright-cyan") }kulupu { $full_name }{ STYLE("reset") } { STYLE("light-gray") }({ $name }){ STYLE("reset") }
    }
msg-org-view-visibility =
    { $visibility ->
        [public] ale li ken lukin
        [limited] jan pi ma ni taso li ken lukin
       *[private] jan pi kulupu ni taso li ken lukin
    }
msg-org-view-member_count = jan { STYLE("bold") }{ $member_count }{ STYLE("reset") } li lon ni
msg-org-view-team_count = kulupu { STYLE("bold") }{ $team_count }{ STYLE("reset") } li lon ni
msg-org-create-invalid_character =
    sitelen pi nimi kulupu li ken ni taso: sitelen Lasina anu sitelen nanpa anu sitelen `-` anu sitelen `_` anu sitelen `.`
      sina wile kepeken sitelen ante la o kepeken nimi `--full-name`
msg-org-create-invalid_starting_character =
    sitelen open pi nimi kulupu li ken sitelen Lasina taso
      sina wile kepeken sitelen ante la o kepeken nimi `--full-name`
msg-org-create-invalid_ending_character =
    sitelen pini pi nimi kulupu li ken ni taso: sitelen Lasina anu sitelen nanpa
      sina wile kepeken sitelen ante la o kepeken nimi `--full-name`
msg-org-create-invalid_consecutive_characters =
    nimi kulupu la sitelen ni li ken ala lon poka sitelen sama: anu sitelen `-` anu sitelen `_` anu sitelen `.`
      sina wile kepeken ni la o kepeken nimi `--full-name`
msg-org-create-success =
    { OPT($full_name) ->
       *[none] { STYLE("bold", "bright-cyan") }kulupu { $name }{ STYLE("reset") }
        [some] { STYLE("bold", "bright-cyan") }kulupu { $full_name }{ STYLE("reset") } { STYLE("light-gray") }({ $name }){ STYLE("reset") }
    } li kama lon. { $visibility ->
        [public] ale li ken lukin e ona
        [limited] jan pi ma ni taso li ken lukin e ona
       *[private] jan pi kulupu ni taso li ken lukin e ona
    }
msg-org-members-no_results = alasa ni la jan ala li lon
msg-org-members-page_number = ni li kulupu nanpa { $page }. kulupu { $total } li lon
msg-org-members-entry =
    { OPT($full_name) ->
       *[none] { STYLE("bold", "bright-cyan") }{ $username }{ STYLE("reset") }
        [some] { STYLE("bold", "bright-cyan") }{ $full_name }{ STYLE("reset") } { STYLE("light-gray") }({ $username }){ STYLE("reset") }
    }
msg-org-visibility-public = sona ni li len ala: sina lon kulupu { STYLE("bold", "bright-cyan") }{ $org_name }{ STYLE("reset") }
msg-org-visibility-private = sona ni li len: sina lon kulupu { STYLE("bold", "bright-cyan") }{ $org_name }{ STYLE("reset") }
msg-org-visibility-set_public = sona ni li kama len ala: sina lon kulupu { STYLE("bold", "bright-cyan") }{ $org_name }{ STYLE("reset") }
msg-org-visibility-set_private = sona ni li kama len: sina lon kulupu { STYLE("bold", "bright-cyan") }{ $org_name }{ STYLE("reset") }
msg-org-visibility-not_member = sina lon ala kulupu { STYLE("bold", "bright-cyan") }{ $org_name }{ STYLE("reset") }
msg-org-label-add-success = poki toki sin { $label } li kama lon
msg-org-label-edit-success = poki toki { $old_label } li kama poki { $label }
msg-org-label-remove-success = poki toki { $label } li weka
msg-org-repo-list-no_results = alasa ni la poki ala li lon
msg-org-repo-list-page_number = ni li kulupu nanpa { $page }. kulupu { $total } li lon
msg-org-team-view =
    { STYLE("bright-blue", "bold") }kulupu { $name }{ STYLE("reset") } pi { STYLE("bold") } kulupu { $org }{ STYLE("reset") } { $admin ->
        [yes] { -dash } { STYLE("bright-red") }ona li ken lawa e kulupu{ STYLE("reset") }
       *[no] { "" }
    }
msg-org-team-view-read_only = ona li ken lukin taso e ni:
msg-org-team-view-read_write = ona li ken ante e ni:
msg-org-team-create-success =
    { STYLE("bright-blue", "bold") }kulupu { $admin ->
        [yes] lawa
       *[no] { "" }
    } { $name }{ STYLE("reset") } li kama lon { STYLE("bold") }kulupu { $org }{ STYLE("reset") }
msg-org-team-delete-confirmation = sina wile ala wile weka e { STYLE("bold") }kulupu { $name }{ STYLE("reset") } tan kulupu { $org }?
    .yes =
        wile
        w
    .no =
        wile ala
        ala
        a
msg-org-team-repo-list-no_results = alasa ni la kulupu ala li lon
msg-org-team-repo-list-page_number = ni li kulupu nanpa { $page }. kulupu { $total } li lon
msg-org-team-repo-add-success = { STYLE("bold", "bright_blue") }kulupu { $team }{ STYLE("reset") } li kama mama e { STYLE("bold") } poki { $org }/{ $repo }{ STYLE("reset") }
msg-org-team-repo-rm-success = { STYLE("bold", "bright_blue") }kulupu { $team }{ STYLE("reset") } li kama mama ala e { STYLE("bold") } poki { $org }/{ $repo }{ STYLE("reset") }
msg-org-team-member-list-no_results = alasa ni la jan ala li lon
msg-org-team-member-list-page_number = ni li kulupu nanpa { $page }. kulupu { $total } li lon
msg-org-team-member-add-success = { STYLE("bold", "bright-cyan") }{ $user }{ STYLE("reset") } li kama lon { STYLE("bold", "bright_blue") }kulupu { $team }{ STYLE("reset") }
msg-org-team-member-rm-success = { STYLE("bold", "bright-cyan") }{ $user }{ STYLE("reset") } li weka tan { STYLE("bold", "bright_blue") }kulupu { $team }{ STYLE("reset") }
msg-issue-create-success = sina open e toki nanpa { $number }: { $title }
msg-issue-view-header =
    toki { STYLE("yellow") }"{ $title }"{ STYLE("dark-grey") } li nanpa { $number }{ STYLE("reset") }
    li tan { STYLE("white") }{ $author }{ STYLE("reset") } li { $state ->
        [open] { STYLE("bright-green") }pini ala{ STYLE("reset") }
        [closed] { STYLE("bright-red") }pini{ STYLE("reset") }
       *[other] $state
    }
msg-issue-view-comment_count = toki { $comments } li lon toki ni
msg-issue-search-total = toki { $issues } li lon poki ni
msg-issue-templates-blank_allowed = nimi "--no-template" li ken
msg-issue-templates-blank_not_allowed = nimi "--no-template" li ken ala
msg-issue-search-entry = toki "{ $title }" li nanpa { $number } li tan { $author }
msg-issue-view-comments-comment_header =
    { OPT($full_name) ->
       *[none] { STYLE("bold", "bright-cyan") }{ $username }{ STYLE("reset") } li toki e ni:
        [some] { STYLE("bold", "bright-cyan") }{ $full_name }{ STYLE("reset") } { STYLE("dark-gray") }({ $username }){ STYLE("reset") } li toki e ni:
    }
msg-issue-view-comments-attachments = ijo { $attachments } li lon toki ni
msg-repo-view-url = o lukin e ona lon { $url }
msg-repo-readme-none = poki ni li jo ala e lipu "README"
msg-issue-edit-title-empty = nimi li ken ala jo e sitelen ala
msg-issue-edit-title-no_newlines = nimi o jo e linja wan taso
msg-repo-clone-preparing = { "     " }mi open...
msg-repo-clone-downloading = { "  " }mi kama jo... { NUMBER($percent, maximumFractionDigits: 2) }% ({ NUMBER($size, maximumFractionDigits: 2) }{ $units })
msg-repo-clone-resolving = { "" }mi kama sona... { NUMBER($percent, maximumFractionDigits: 2) }%
msg-repo-clone-finishing_up = { "     " }mi pini...
msg-repo-clone-success = poki weka { $repo } li kama lon poki poka { $path }
msg-repo-delete-confirmation_prompt = sina wile ala wile weka e poki { $owner }/{ $name }?
    .yes =
        wile
        w
    .no =
        wile ala
        ala
        a
msg-repo-delete-success = { $owner }/{ $repo } li weka
msg-repo-delete-cancelled = ona li weka ala
msg-repo-label-view-archived = (majuna)
msg-repo-label-view-no_description = (toki ala)
msg-repo-label-create-success = poki toki { $label } li kama lon
msg-repo-label-delete-success = poki toki { $label } li kama weka
msg-repo-label-edit-success = poki toki { $label } li kama ante
msg-user-search-page_zero = kulupu nanpa 0 li lon ala
msg-user-search-fail = alasa li pakala
msg-user-view-joined_on = ona li kama lon ma ni lon { STYLE("bold") }tenpo { DATETIME($joined, dateStyle: "medium") }{ STYLE("reset") }
msg-user-follow-success = sina lukin e { $username }
msg-user-unfollow-success = sina lukin ala e { $username }
msg-user-following-none-other = { $user } li lukin e jan ala
msg-user-following-none-self = sina lukin e jan ala
msg-user-following-other = { $user } li lukin e ni:
msg-user-following-self = sinai lukin e ni:
msg-user-followers-none-other = jan ala li lukin e { $user }
msg-user-followers-none-self = jan ala li lukin e sina :(
msg-user-followers-other = jan ni li lukin e { $user }:
msg-user-followers-self = jan ni li lukin e sina:
msg-user-block-success = sina weka e { $user } tan lukin sina
msg-user-unblock-success = sina weka ala e { $user } tan lukin sina
msg-user-repos-none-other = { $name } li jo ala e poki
msg-user-repos-none-self = sina jo ala e poki
msg-user-orgs-none-other = { $user } li lon kulupu ala
msg-user-orgs-none-self = sina lon kulupu ala
msg-user-orgs-count = kulupu { $organizations }
-dash =
    { IS_MINIMAL() ->
        [yes] -
       *[no] —
    }
msg-org-team-view-perms-wiki = lipu sona
msg-org-team-view-perms-ext_wiki = lipu sona pi ma ante
msg-org-team-view-perms-issues = toki
msg-org-team-view-perms-ext_issues = toki pi ma ante
msg-org-team-view-perms-pulls = ante wile
msg-org-team-view-perms-projects = sinpin pali
msg-org-team-view-perms-code = toki ilo
msg-org-team-view-perms-releases = pana
msg-org-team-view-perms-packages = pana poki
msg-issue-create-no_templates = poki { $owner }/{ $repo } li kepeken ala nasin toki
msg-issue-create-templates_required =
    poki { $owner }/{ $repo } la o kepeken nasin toki
    sina ken kepeken ona kepeken nimi `--template <NIMI>`
msg-issue-create-templates_enabled =
    poki { $owner }/{ $repo } li kepeken nasin toki
    sina ken kepeken ona kepeken nimi `--template <NIMI>`
    anu ken kepeken ala ona kepeken nimi `--no-template`
msg-issue-templates-none = nasin toki ala li lon poki ni
msg-issue-close-success = sina pini e toki nanpa { $number }: { $title }
msg-pr-couldnt_guess = mi sona ala e nanpa pi ante wile. o pana e nanpa
msg-pr-not_found = mi sona ala e ante wile ni
msg-pr-view-header =
    { STYLE("yellow") }ante wile "{ $title }" { STYLE("dark-grey") } li { $number }{ STYLE("reset") }
    li tan { STYLE("white") }{ $username }{ STYLE("reset") } { $state ->
        [draft] { STYLE("light-grey") }. ante li pini ala{ STYLE("reset") }
        [open] { STYLE("bright-green") }li pini ala{ STYLE("reset") }
        [merged] { STYLE("bright-magenta") }. ante li tawa mama ona{ STYLE("reset") }
        [closed] { STYLE("bright-red") }li pini{ STYLE("reset") }
       *[other] $state
    } { -dash } { STYLE("bright-green") }linja { $additions } li kama { STYLE("reset") }, { STYLE("bright-red") }linja { $deletions } li weka{ STYLE("reset") }
    { OPT($head_branch) ->
       *[none] ante li tawa `{ $base_branch }`
        [some] ante li kama tan `{ $head_branch }` li tawa `{ $base_branch }`
    }
msg-pr-view-comment_count = { $comments } toki li lon ante wile ni
msg-pr-status-merged = { STYLE("bright-magenta") }ante li tawa mama ona{ STYLE("reset") }. { $merged_by } li wan e ona lon tenpo { DATETIME($created_at, dateStyle: "long", timeStyle: "long") }
msg-pr-status-header =
    { $state ->
        [draft] { STYLE("light-grey") }ona li sin{ STYLE("reset") } { -dash } la sina ken ala wan e ona
        [open]
            { STYLE("bright_green") }ona li pini ala{ STYLE("reset") } { $mergeable ->
               *[yes] li ken kama wa
                [no] { STYLE("bright-red") } li ken ala kama wan{ STYLE("reset") }
            }
        [closed] { STYLE("bright-red") }ona li pini{ STYLE("reset") } { -dash } sina wile wan e ona la o open sin e ona
       *[other] nasa
    }
msg-pr-status-entry =
    { $state ->
        [success] { STYLE("bright_green") }pona{ STYLE("reset") }
        [pending] { STYLE("yellow") }open ala{ STYLE("reset") }
        [warning] { STYLE("bright_yellow") }ike ken { STYLE("reset") }
        [failure] { STYLE("bright_red") }ike{ STYLE("reset") }
        [error] { STYLE("bright_red") }pakala{ STYLE("reset") }
       *[other] nasa
    } { -dash } { $context }
msg-pr-create-cross_instance = ante wile o lon ma wan taso; mama li lon ma { $base_instance }, taso ante li lon ma { $head_instance }
msg-pr-create-success = sina open e ante wile nanpa { $number }: { $title }
msg-pr-create-agit_success = sina open e ante wile: { $title }
msg-pr-create-agit_push_cfg_question =
    sina wile ala wile e ni:
    nasin pi ilo Git li ante tawa ni: nasin `git push` li ken, lon ante wile ni
msg-pr-create-agit_push_cfg_prompt = (w/A/?)
    .yes =
        wile
        w
    .no =
        wile ala
        ala
        a
    .help =
        seme
        s
        ?
msg-pr-create-agit_force_push_warning =
    { STYLE("bold") }o sona e ni:{ STYLE("reset") }
      nasin AGit la `git push --force[-with-lease]` li ken ala.
      o kepeken nasin `git push -o force=true`.
msg-pr-create-agit_push_cfg_help = ni li ante e nasin ni:
msg-pr-merge-default_message = Reviewed-on: { $pr_url }
msg-pr-merge-success = ante wile nanpa { $number } \"{ $title }\" li kama tawa `{ $base_branch }`
msg-pr-checkout-dirty = ante pi awen ala li lon poka la mi ken ala lukin e ante wile ni
msg-pr-checkout-not_fork = poki { $repo } li jo ala e poki mama
msg-pr-search-entry = ante wile "{ $title }" li nanpa { $number } li tan { $author }
msg-repo-fallback_host-invalid_url = o sona: `FJ_FALLBACK_HOST` li pakala
msg-repo-arg_no_owner = nimi poki o kepeken nasin nimi ni: [MA/]JAN/POKI anu [MA/]KULUPU/POKI
