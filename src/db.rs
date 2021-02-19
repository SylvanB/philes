use pickledb::PickleDb;

use crate::uploads::FileInfo;

pub(crate) struct Db {
    db: PickleDb,
}

impl Db {
    pub fn get_all(self) -> Vec<FileInfo> {
        let ids = self.db.get_all();
        let results = ids
            .iter()
            .map(|k| (k.to_owned(), self.db.get(k)))
            .filter(|(_, v)| v.is_some())
            .map(|(k, v)| FileInfo {
                id: k,
                location: v.unwrap(),
            })
            .collect();

        results
    }

    pub fn new() -> Self {
        Db {
            db: PickleDb::load(
                "data.db",
                pickledb::PickleDbDumpPolicy::AutoDump,
                pickledb::SerializationMethod::Json,
            )
            .unwrap(),
        }
    }
}
