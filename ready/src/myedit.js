import * as React from 'react';
import {
  Edit,
  SimpleForm,
  ImageInput,
  Confirm,
  useDataProvider,
} from 'react-admin';
import { useMutation } from 'react-query';

const MyEdit = (props) => {
  const [removeImage, setRemoveImage] = React.useState(null);
  const [showModal, setShowModal] = React.useState(false);
  const dataProvider = useDataProvider();
  const { mutate } = useMutation();

  return (
    <Edit {...props}>
      <SimpleForm>
        <ImageInput
          source="images"
          src="image"
          validateFileRemoval={(file, _record) => {
            const promise = new Promise((_resolve, reject) => {
              setRemoveImage({
                fileName: `Image ID: ${file.id}`,
                delete: async (result) => {
                  await mutate(['deleteImages', { ids: [file.id] }], () =>
                    dataProvider.deleteImages({ ids: [file.id] })
                  );
                  return _resolve(result);
                },
                cancel: reject,
              });
            });
            setShowModal(true);
            return promise.then((result) => {
              console.log('Image removed!');
            });
          }}
        />
        <Confirm
          isOpen={showModal}
          title="Delete image"
          content={`${removeImage ? removeImage.fileName : ''} will be deleted`}
          onConfirm={() => {
            setShowModal(false);
            removeImage && removeImage.delete();
          }}
          onClose={() => {
            setShowModal(false);
            removeImage && removeImage.cancel();
          }}
        />
      </SimpleForm>
    </Edit>
  );
};
