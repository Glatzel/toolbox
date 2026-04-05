job "ghar-linux-dev-small" {
  type = "batch"

  parameterized {
      payload       = "optional"
      meta_required = ["OWNER", "REPO", "TOKEN", "ID"]
  }

  group "runner" {
    task "start-vm" {
      driver = "raw_exec"
      config {
        command = "msb"
        args    = [
        "run",
        "--debug",
        "--replace",
        "--env", "GH_OWNER=${NOMAD_META_OWNER}",
        "--env", "GH_REPOSITORY=${NOMAD_META_REPO}",
        "--env", "GH_TOKEN=${NOMAD_META_TOKEN}",
        "--env", "EPHEMERAL=true",
        "--env", "RUNNER_LABELS=self-hosted,ghar-linux-dev-small",
        "--name", "ghar-linux-dev-small${NOMAD_META_ID}",
        "ghcr.io/glatzel/ghar-linux-dev"
        ]
      }
    }
    task "cleanup" {
      driver = "raw_exec"
      lifecycle {
        hook    = "poststop"
      }
      config {
        command = "msb"
        args    = ["rm", "--force", "ghar-linux-dev-small${NOMAD_META_ID}"]
      }
    }
  }
}
