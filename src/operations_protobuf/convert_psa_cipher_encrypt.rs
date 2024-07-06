// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use super::convert_psa_algorithm;
use super::generated_ops::psa_cipher_encrypt::{
    Operation as OperationProto, Result as ResultProto,
};
use crate::operations::psa_cipher_encrypt::{Operation, Result};
use crate::requests::ResponseStatus;
use std::convert::TryFrom;

impl TryFrom<OperationProto> for Operation {
    type Error = ResponseStatus;

    fn try_from(proto_op: OperationProto) -> std::result::Result<Self, Self::Error> {
        Ok(Operation {
            key_name: proto_op.key_name,
            alg: convert_psa_algorithm::i32_to_cipher(proto_op.alg)?,
            plaintext: proto_op.plaintext.into(),
        })
    }
}

impl TryFrom<Operation> for OperationProto {
    type Error = ResponseStatus;

    fn try_from(op: Operation) -> std::result::Result<Self, Self::Error> {
        Ok(OperationProto {
            key_name: op.key_name,
            alg: convert_psa_algorithm::cipher_to_i32(op.alg),
            plaintext: op.plaintext.to_vec(),
        })
    }
}

impl TryFrom<ResultProto> for Result {
    type Error = ResponseStatus;

    fn try_from(proto_result: ResultProto) -> std::result::Result<Self, Self::Error> {
        Ok(Result {
            ciphertext: proto_result.ciphertext.into(),
        })
    }
}

impl TryFrom<Result> for ResultProto {
    type Error = ResponseStatus;

    fn try_from(result: Result) -> std::result::Result<Self, Self::Error> {
        Ok(ResultProto {
            ciphertext: result.ciphertext.to_vec(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::super::generated_ops::psa_cipher_encrypt::{
        Operation as OperationProto, Result as ResultProto,
    };
    use super::super::{Convert, ProtobufConverter};
    use super::convert_psa_algorithm;
    use crate::operations::psa_algorithm::Cipher;
    use crate::operations::psa_cipher_encrypt::{Operation, Result};
    use std::convert::TryInto;
    static CONVERTER: ProtobufConverter = ProtobufConverter {};
    use crate::operations::{NativeOperation, NativeResult};
    use crate::requests::{request::RequestBody, response::ResponseBody, Opcode};

    #[test]
    fn cipher_proto_to_op() {
        let mut proto: OperationProto = Default::default();
        let message = vec![0x11, 0x22, 0x33];
        let key_name = "test name".to_string();
        let proto_alg = Cipher::StreamCipher;
        proto.plaintext = message.clone();
        proto.alg = convert_psa_algorithm::cipher_to_i32(proto_alg);
        proto.key_name = key_name.clone();

        let op: Operation = proto.try_into().expect("Failed to convert");

        assert_eq!(*op.plaintext, message);
        assert_eq!(op.key_name, key_name);
    }

    #[test]
    fn cipher_op_to_proto() {
        let message = vec![0x11, 0x22, 0x33];
        let key_name = "test name".to_string();

        let op = Operation {
            plaintext: message.clone().into(),
            alg: Cipher::StreamCipher,
            key_name: key_name.clone(),
        };

        let proto: OperationProto = op.try_into().expect("Failed to convert");

        assert_eq!(proto.plaintext, message);
        assert_eq!(proto.key_name, key_name);
    }

    #[test]
    fn cipher_proto_to_resp() {
        let mut proto: ResultProto = Default::default();
        let ciphertext = vec![0x11, 0x22, 0x33];
        proto.ciphertext = ciphertext.clone();

        let result: Result = proto.try_into().expect("Failed to convert");

        assert_eq!(*result.ciphertext, ciphertext);
    }

    #[test]
    fn cipher_resp_to_proto() {
        let ciphertext = vec![0x11, 0x22, 0x33];
        let result = Result {
            ciphertext: ciphertext.clone().into(),
        };

        let proto: ResultProto = result.try_into().expect("Failed to convert");

        assert_eq!(proto.ciphertext, ciphertext);
    }

    #[test]
    fn psa_encrypt_message_op_e2e() {
        let name = "test name".to_string();
        let op = Operation {
            key_name: name,
            alg: Cipher::StreamCipher,
            plaintext: vec![0x11, 0x22, 0x33].into(),
        };

        let body = CONVERTER
            .operation_to_body(NativeOperation::PsaCipherEncrypt(op))
            .expect("Failed to convert to body");

        let _ = CONVERTER
            .body_to_operation(body, Opcode::PsaCipherEncrypt)
            .expect("Failed to convert to operation");
    }

    #[test]
    fn resp_cipher_encrypt_e2e() {
        let result = Result {
            ciphertext: vec![0x11, 0x22, 0x33].into(),
        };
        let body = CONVERTER
            .result_to_body(NativeResult::PsaCipherEncrypt(result))
            .expect("Failed to convert request");

        assert!(CONVERTER
            .body_to_result(body, Opcode::PsaCipherEncrypt)
            .is_ok());
    }

    #[test]
    fn result_from_mangled_resp_body() {
        let resp_body =
            ResponseBody::from_bytes(vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
        assert!(CONVERTER
            .body_to_result(resp_body, Opcode::PsaCipherEncrypt)
            .is_err());
    }

    #[test]
    fn op_from_mangled_req_body() {
        let req_body =
            RequestBody::from_bytes(vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);

        assert!(CONVERTER
            .body_to_operation(req_body, Opcode::PsaCipherEncrypt)
            .is_err());
    }
}
