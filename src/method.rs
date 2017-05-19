use std::slice::SliceIndex;
use std::convert::From;

pub mod conn {
    pub enum Method {
        Start(),
        Secure(),
        Tune(),
        Open(),
        Close(),
        Blocked(), Unblocked()
    }

    pub enum Response {
        StartOk(), SecureOk(), TuneOk(),
        OpenOk(), CloseOk()
    }
}

pub mod channel {
    pub enum Method {
        Open(),
        Flow(),
        Close()
    }

    pub enum Response {
        OpenOk(), FlowOk(), CloseOk()
    }
}

pub mod access {
    pub enum Method { Request() }
    pub enum Response { RequestOk() }
}

pub mod exchange {
    pub enum Method {
        Declare(),
        Delete(),
        Bind(),
        Unbind()
    }

    pub enum Response {
        DeclareOk(),
        DeleteOk(),
        BindOk(),
        UnbindOk()
    }

}

pub mod queue {
    pub enum Method {
        Declare(),
        Bind(),
        Purge(),
        Delete(),
        Unbind()
    }

    pub enum Response {
        DeclareOk(),
        BindOk(),
        PurgeOk(),
        DeleteOk(),
        UnbindOk()
    }
}

pub mod basic {
    pub enum Method {
        Qos(),
        Consume(),
        Cancel(),
        Publish(),
        Return(),
        Deliver(),
        Get(),
        GetEmpty(),
        Ack(),
        Reject(),
        RecoverAsync(),
        Recover(),
        Nack()
    }

    pub enum Response {
        QosOk(),
        ConsumeOk(),
        CancelOk(),
        GetOk(),
        RecoverOk()
    }
}

pub mod trans {
    pub enum Method {
        Select(),
        Commit(),
        Rollback()
    }

    pub enum Response {
        SelectOk(),
        CommitOk(),
        RollbackOk()
    }
}

pub mod confim {
    pub enum Method { Select() }
    pub enum Response { SelectOk() }
}

// pub enum Method {
//     conn::Method,
//     channel::Method,
//     access::Method,
//     exchange::Method,
//     queue::Method,
//     basic::Method,
//     trans::Method,
//     confirm::Method
// }

// pub enum Response {
//     conn::Response,
//     channel::Response,
//     access::Response,
//     exchange::Response,
//     queue::Response,
//     basic::Response,
//     trans::Response,
//     confirm::Response
// }
