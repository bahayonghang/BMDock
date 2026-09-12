pub const OWNED_WORKSPACE_ID: &str = "bmdock-workspace";
pub const OWNED_KIND: &str = "bmdock_owned";
pub const POLICY_NON_OWNED_PROJECT: &str = "Only the generated fixture project is allowed";
const FIXTURE_PROJECT: &str = "bmdock-fixture";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RouteState {
    pub workspace: Option<String>,
    pub project: Option<String>,
}

impl RouteState {
    pub fn select_fixture(&mut self) {
        self.workspace = Some(OWNED_WORKSPACE_ID.to_owned());
        self.project = Some(FIXTURE_PROJECT.to_owned());
    }
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub struct WorkspaceRecordDto {
    pub id: String,
    pub kind: String,
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub struct ProjectRecordDto {
    pub id: String,
    pub workspace: String,
    pub kind: String,
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub struct ProjectCatalogDto {
    pub workspaces: Vec<WorkspaceRecordDto>,
    pub projects: Vec<ProjectRecordDto>,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub cross_project_search_allowed: bool,
    pub implicit_current_project_writes: bool,
    pub cloud_or_credential_required: bool,
    pub local_offline: bool,
    pub files_written: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct ExplicitRouteArgs {
    pub workspace: String,
    pub project: String,
}

pub fn owned_catalog() -> ProjectCatalogDto {
    ProjectCatalogDto {
        workspaces: vec![WorkspaceRecordDto {
            id: OWNED_WORKSPACE_ID.to_owned(),
            kind: OWNED_KIND.to_owned(),
        }],
        projects: vec![ProjectRecordDto {
            id: FIXTURE_PROJECT.to_owned(),
            workspace: OWNED_WORKSPACE_ID.to_owned(),
            kind: OWNED_KIND.to_owned(),
        }],
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        cross_project_search_allowed: false,
        implicit_current_project_writes: false,
        cloud_or_credential_required: false,
        local_offline: true,
        files_written: false,
    }
}

#[allow(dead_code)]
pub fn require_explicit_fixture_route(route: &ExplicitRouteArgs) -> Result<(), &'static str> {
    if route.workspace == OWNED_WORKSPACE_ID && route.project == FIXTURE_PROJECT {
        Ok(())
    } else {
        Err(POLICY_NON_OWNED_PROJECT)
    }
}

#[cfg(test)]
pub fn is_bmdock_owned_project(project: &str) -> bool {
    project == FIXTURE_PROJECT
}

#[cfg(test)]
pub fn owned_project_or_policy(project: &str) -> Result<(), &'static str> {
    if is_bmdock_owned_project(project) {
        Ok(())
    } else {
        Err(POLICY_NON_OWNED_PROJECT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_constant_matches_ipc() {
        assert_eq!(FIXTURE_PROJECT, crate::ipc::FIXTURE_PROJECT);
    }

    #[test]
    fn default_catalog_is_fixture_only_and_local() {
        let catalog = owned_catalog();
        assert_eq!(catalog.workspaces.len(), 1);
        assert_eq!(catalog.workspaces[0].id, OWNED_WORKSPACE_ID);
        assert_eq!(catalog.workspaces[0].kind, OWNED_KIND);
        assert_eq!(catalog.projects.len(), 1);
        assert_eq!(catalog.projects[0].id, FIXTURE_PROJECT);
        assert_eq!(catalog.projects[0].workspace, OWNED_WORKSPACE_ID);
        assert_eq!(catalog.projects[0].kind, OWNED_KIND);
        assert!(!catalog.scanned_user_obsidian_vault);
        assert!(!catalog.scanned_user_basic_memory_home);
        assert!(!catalog.cross_project_search_allowed);
        assert!(!catalog.implicit_current_project_writes);
        assert!(!catalog.cloud_or_credential_required);
        assert!(catalog.local_offline);
        assert!(!catalog.files_written);
        let json = serde_json::to_value(&catalog).unwrap();
        assert_ne!(json["workspaces"][0]["id"], r"C:\Users\someone\vault");
        assert_ne!(json["projects"][0]["id"], "/home/someone/.basic-memory");
        assert!(json.get("search").is_none());
        assert!(json.get("query").is_none());
    }

    #[test]
    fn user_vault_and_global_home_are_not_owned() {
        for project in [
            r"C:\Users\someone\Documents\vault",
            r"C:\Users\someone\Documents\Obsidian",
            r"C:\Users\someone\.basic-memory",
            "/home/someone/.basic-memory",
            "",
            "../vault",
        ] {
            assert!(
                !is_bmdock_owned_project(project),
                "project should not be owned: {project}"
            );
            assert_eq!(
                owned_project_or_policy(project).unwrap_err(),
                POLICY_NON_OWNED_PROJECT
            );
        }
        assert_eq!(owned_project_or_policy(FIXTURE_PROJECT), Ok(()));
    }

    #[test]
    fn explicit_route_requires_fixture_and_owned_workspace() {
        require_explicit_fixture_route(&ExplicitRouteArgs {
            workspace: OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
        })
        .unwrap();
        assert_eq!(
            require_explicit_fixture_route(&ExplicitRouteArgs {
                workspace: OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\vault".to_owned(),
            })
            .unwrap_err(),
            POLICY_NON_OWNED_PROJECT
        );
        assert_eq!(
            require_explicit_fixture_route(&ExplicitRouteArgs {
                workspace: "user-home".to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
            })
            .unwrap_err(),
            POLICY_NON_OWNED_PROJECT
        );
        let missing_project =
            serde_json::from_str::<ExplicitRouteArgs>(r#"{"workspace":"bmdock-workspace"}"#);
        assert!(missing_project.is_err());
        let extra_path = serde_json::from_str::<ExplicitRouteArgs>(
            r#"{"workspace":"bmdock-workspace","project":"bmdock-fixture","path":"C:\\vault"}"#,
        );
        assert!(extra_path.is_err());
    }

    #[test]
    fn select_fixture_does_not_imply_writes() {
        let mut route = RouteState::default();
        assert_eq!(route.project, None);
        route.select_fixture();
        assert_eq!(route.project.as_deref(), Some(FIXTURE_PROJECT));
        assert_eq!(route.workspace.as_deref(), Some(OWNED_WORKSPACE_ID));
        let catalog = owned_catalog();
        assert!(!catalog.implicit_current_project_writes);
        assert!(!catalog.files_written);
    }
}
