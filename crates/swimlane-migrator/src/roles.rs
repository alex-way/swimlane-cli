use swimlane::roles::{Access, PermissionMatrix, Role};

use crate::equality::{Difference, LooksLike};
use crate::{MigrationPlan, SwimlaneMigrator, SwimlaneMigratorError};

impl LooksLike for Role {
    fn differences(&self, other: &Self) -> Vec<Difference> {
        let mut differences = vec![];

        // Add warning if the name is different
        push_difference!(differences, "name", &self.name, &other.name);
        push_difference!(differences, "disabled", &self.disabled, &other.disabled);
        push_difference!(differences, "description", &self.description, &other.description, optional: true);
        push_difference!(differences, "users", &self.users, &other.users, vec: true);

        differences
    }

    fn is_same_resource(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl LooksLike for Access {
    fn differences(&self, other: &Self) -> Vec<Difference> {
        let mut differences = vec![];

        for (key, value) in self.permissions.iter() {
            if let Some(other_value) = other.permissions.get(key) {
                if value != other_value {
                    differences.push(Difference::UpdatingField {
                        field: key.to_string(),
                        current_value: value.to_string(),
                        new_value: other_value.to_string(),
                    });
                }
            } else {
                differences.push(Difference::AddingItem {
                    field: "permissions".to_string(),
                    item: key.clone(),
                });
            }
        }

        for (key, _) in other.permissions.iter() {
            if !self.permissions.contains_key(key) {
                differences.push(Difference::RemovingItem {
                    field: "permissions".to_string(),
                    item: key.clone(),
                });
            }
        }

        differences
    }

    fn is_same_resource(&self, _other: &Self) -> bool {
        // The Access itself has no unique identifier
        unreachable!()
    }
}

impl LooksLike for PermissionMatrix {
    fn differences(&self, other: &Self) -> Vec<Difference> {
        // The PermissionMatrix itself has no unique identifier so we assume it's always the same
        let mut differences = vec![];
        for (_, value) in self.permissions.iter() {
            if let Some(other_permission) =
                other.permissions.iter().find(|(_, v)| v.name == value.name)
            {
                let other_permission = other_permission.1;
                if value.access != other_permission.access {
                    differences.push(Difference::UpdatingField {
                        field: format!("permissions.{}.access", value.name),
                        current_value: value.access.to_string(),
                        new_value: other_permission.access.to_string(),
                    });
                }

                // Also check each iteration of the field permissions
                let field_differences = value.fields.differences(&other_permission.fields);
                if !field_differences.is_empty() {
                    differences.push(Difference::UpdatingComplexField {
                        field: format!("permissions.{}.fields", value.name),
                    });
                }
            } else {
                differences.push(Difference::AddingItem {
                    field: "permissions".to_string(),
                    item: value.name.clone(),
                });
            }
        }

        for (_, value) in other.permissions.iter() {
            let permission_exists = self.permissions.iter().any(|(_, v)| v.name == value.name);
            if !permission_exists {
                differences.push(Difference::RemovingItem {
                    field: "permissions".to_string(),
                    item: value.name.clone(),
                });
            }
        }

        differences
    }

    fn is_same_resource(&self, _other: &Self) -> bool {
        // The PermissionMatrix itself has no unique identifier
        unreachable!()
    }
}

impl SwimlaneMigrator {
    pub async fn get_roles_to_migrate(
        &self,
    ) -> Result<Vec<MigrationPlan<Role>>, SwimlaneMigratorError> {
        let source_roles_future = self.from.get_roles();
        let target_roles_future = self.to.get_roles();

        self.get_resources_to_migrate(source_roles_future, target_roles_future)
            .await
    }

    pub async fn migrate_roles(&self) -> Result<(), SwimlaneMigratorError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use swimlane::roles::{Access, Permission, PermissionType};

    use super::*;

    #[test]
    fn test_permission_matrix_identical_permissions_different_ids_difference_is_0() {
        let mut field_perms = HashMap::new();

        field_perms.insert("field 1".to_string(), 0);
        field_perms.insert("field 2".to_string(), 0);

        let matrix1_permissions = vec![Permission {
            _type: "permission".to_string(),
            id: "1".to_string(),
            type_: PermissionType::Role,
            name: "TIER-1".to_string(),
            access: 0,
            fields: Access {
                _type: "access".to_string(),
                permissions: field_perms.clone(),
            },
        }];

        let matrix1_permissions: HashMap<String, Permission> = matrix1_permissions
            .into_iter()
            .map(|p| (p.id.clone(), p))
            .collect();

        let matrix2_permissions = vec![Permission {
            _type: "permission".to_string(),
            id: "3".to_string(),
            type_: PermissionType::Role,
            name: "TIER-1".to_string(),
            access: 0,
            fields: Access {
                _type: "access".to_string(),
                permissions: field_perms.clone(),
            },
        }];

        let matrix2_permissions: HashMap<String, Permission> = matrix2_permissions
            .into_iter()
            .map(|p| (p.id.clone(), p))
            .collect();

        let matrix1 = PermissionMatrix {
            _type: "PermissionMatrix".to_string(),
            permissions: matrix1_permissions,
        };

        let matrix2 = PermissionMatrix {
            _type: "PermissionMatrix".to_string(),
            permissions: matrix2_permissions,
        };

        let differences = matrix1.differences(&matrix2);
        assert_eq!(differences.len(), 0);
    }

    #[test]
    fn test_permission_matrix_missing_field_increments_difference() {
        let mut field_perms = HashMap::new();

        field_perms.insert("field 1".to_string(), 0);
        field_perms.insert("field 2".to_string(), 0);

        let matrix1_permissions = vec![Permission {
            _type: "permission".to_string(),
            id: "1".to_string(),
            type_: PermissionType::Role,
            name: "TIER-1".to_string(),
            access: 0,
            fields: Access {
                _type: "access".to_string(),
                permissions: field_perms.clone(),
            },
        }];

        let matrix1_permissions: HashMap<String, Permission> = matrix1_permissions
            .into_iter()
            .map(|p| (p.id.clone(), p))
            .collect();

        let matrix2_permissions: HashMap<String, Permission> = HashMap::new();

        let matrix1 = PermissionMatrix {
            _type: "PermissionMatrix".to_string(),
            permissions: matrix1_permissions,
        };

        let matrix2 = PermissionMatrix {
            _type: "PermissionMatrix".to_string(),
            permissions: matrix2_permissions,
        };

        let differences = matrix1.differences(&matrix2);
        assert_eq!(differences.len(), 1);
    }

    #[test]
    fn test_permission_matrix_extra_field_increments_difference() {
        let mut field_perms = HashMap::new();

        field_perms.insert("field 1".to_string(), 0);
        field_perms.insert("field 2".to_string(), 0);

        let matrix1_permissions: HashMap<String, Permission> = HashMap::new();

        let matrix2_permissions = vec![Permission {
            _type: "permission".to_string(),
            id: "1".to_string(),
            type_: PermissionType::Role,
            name: "TIER-1".to_string(),
            access: 0,
            fields: Access {
                _type: "access".to_string(),
                permissions: field_perms.clone(),
            },
        }];

        let matrix2_permissions: HashMap<String, Permission> = matrix2_permissions
            .into_iter()
            .map(|p| (p.id.clone(), p))
            .collect();

        let matrix1 = PermissionMatrix {
            _type: "PermissionMatrix".to_string(),
            permissions: matrix1_permissions,
        };

        let matrix2 = PermissionMatrix {
            _type: "PermissionMatrix".to_string(),
            permissions: matrix2_permissions,
        };

        let differences = matrix1.differences(&matrix2);
        assert_eq!(differences.len(), 1);
    }

    #[test]
    fn test_permission_matrix_same_permission_name_different_permission_level_incremements_difference(
    ) {
        let field_perms = HashMap::new();

        let matrix1_permissions = vec![Permission {
            _type: "permission".to_string(),
            id: "1".to_string(),
            type_: PermissionType::Role,
            name: "TIER-1".to_string(),
            access: 0,
            fields: Access {
                _type: "access".to_string(),
                permissions: field_perms,
            },
        }];

        let mut matrix2_permissions = matrix1_permissions.clone();

        let matrix1_permissions: HashMap<String, Permission> = matrix1_permissions
            .into_iter()
            .map(|p| (p.id.clone(), p))
            .collect();

        matrix2_permissions[0].access = 1;

        let matrix2_permissions: HashMap<String, Permission> = matrix2_permissions
            .into_iter()
            .map(|p| (p.id.clone(), p))
            .collect();

        let matrix1 = PermissionMatrix {
            _type: "PermissionMatrix".to_string(),
            permissions: matrix1_permissions,
        };

        let matrix2 = PermissionMatrix {
            _type: "PermissionMatrix".to_string(),
            permissions: matrix2_permissions,
        };

        let differences = matrix1.differences(&matrix2);
        assert_eq!(differences.len(), 1);
    }

    #[test]
    fn test_permission_matrix_same_permission_name_different_field_permissions_incremements_difference(
    ) {
        let mut field_perms_1 = HashMap::new();
        field_perms_1.insert("field 1".to_string(), 0);

        let mut field_perms_2 = HashMap::new();
        field_perms_2.insert("field 1".to_string(), 1);

        let matrix1_permissions = vec![Permission {
            _type: "permission".to_string(),
            id: "1".to_string(),
            type_: PermissionType::Role,
            name: "TIER-1".to_string(),
            access: 0,
            fields: Access {
                _type: "access".to_string(),
                permissions: field_perms_1,
            },
        }];

        let matrix1_permissions: HashMap<String, Permission> = matrix1_permissions
            .into_iter()
            .map(|p| (p.id.clone(), p))
            .collect();

        let matrix2_permissions = vec![Permission {
            _type: "permission".to_string(),
            id: "1".to_string(),
            type_: PermissionType::Role,
            name: "TIER-1".to_string(),
            access: 0,
            fields: Access {
                _type: "access".to_string(),
                permissions: field_perms_2,
            },
        }];

        let matrix2_permissions: HashMap<String, Permission> = matrix2_permissions
            .into_iter()
            .map(|p| (p.id.clone(), p))
            .collect();

        let matrix1 = PermissionMatrix {
            _type: "PermissionMatrix".to_string(),
            permissions: matrix1_permissions,
        };

        let matrix2 = PermissionMatrix {
            _type: "PermissionMatrix".to_string(),
            permissions: matrix2_permissions,
        };

        let differences = matrix1.differences(&matrix2);
        assert_eq!(differences.len(), 1);
    }
}
