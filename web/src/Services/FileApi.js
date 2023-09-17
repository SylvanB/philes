class FileApi {
    // async getById(id) {
    //     return {};
    // }

    async getAll() {
        let files = await fetch("/files", {
            method: "GET",
        })
            .then(async (resp) => resp.json())
            .catch((err) => []);

        console.log(files);
        return files;
    }

    // async deleteById(id) {
    //     return true;
    // }

    async upsert(newFiles) {
        let formData = new FormData();

        for (const file in newFiles) {
            formData.append(`file${file}`, newFiles[file]);
        }

        let req = new Request("/files", {
            method: "POST",
            body: formData,
        });

        let uploadedFiles = await fetch(req)
            .then(async (resp) => {
                return await resp.json();
            })
            .catch((err) => {
                return false;
            });

        console.log(uploadedFiles);
        return uploadedFiles;
    }
}

export default FileApi;
