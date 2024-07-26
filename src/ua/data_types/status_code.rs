use std::{ffi::CStr, fmt};

use open62541_sys::{
    UA_StatusCode, UA_StatusCode_isBad, UA_StatusCode_isGood, UA_StatusCode_isUncertain,
    UA_StatusCode_name,
};

crate::data_type!(StatusCode);

impl StatusCode {
    /// Creates wrapper by taking ownership of `src`.
    #[must_use]
    pub(crate) const fn new(src: UA_StatusCode) -> Self {
        Self(src)
    }

    /// Checks if status code is good.
    ///
    /// Good status codes indicate that the operation was successful and the associated results may
    /// be used.
    ///
    /// Note: This only checks the _severity_ of the status code. If you want to see if the code is
    /// exactly the single status code [`GOOD`](Self::GOOD), use comparison instead:
    ///
    /// ```
    /// use open62541::ua;
    ///
    /// # let status_code = ua::StatusCode::GOOD;
    /// if status_code == ua::StatusCode::GOOD {
    ///     //
    /// }
    /// ```
    #[must_use]
    pub fn is_good(&self) -> bool {
        unsafe { UA_StatusCode_isGood(self.0) }
    }

    /// Checks if status code is uncertain.
    ///
    /// Uncertain status codes indicate that the operation was partially successful and that
    /// associated results might not be suitable for some purposes.
    #[must_use]
    pub fn is_uncertain(&self) -> bool {
        unsafe { UA_StatusCode_isUncertain(self.0) }
    }

    /// Checks if status code is bad.
    ///
    /// Bad status codes indicate that the operation failed and any associated results cannot be
    /// used.
    #[must_use]
    pub fn is_bad(&self) -> bool {
        unsafe { UA_StatusCode_isBad(self.0) }
    }

    /// Gets name of status code.
    ///
    /// This returns the human-readable name of the status code, e.g. `BadNotWritable`.
    ///
    /// # Examples
    ///
    /// ```
    /// use open62541::ua;
    ///
    /// assert_eq!(ua::StatusCode::GOOD.name(), "Good");
    /// ```
    #[must_use]
    pub fn name(&self) -> &'static str {
        status_code_name(self.0)
    }

    /// Gets status code.
    pub(crate) const fn code(&self) -> UA_StatusCode {
        self.0
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Gets statically allocated string with name of status code.
fn status_code_name(status_code: UA_StatusCode) -> &'static str {
    let name = unsafe { UA_StatusCode_name(status_code) };
    // SAFETY: `name` is a pointer to a valid NUL-terminated C string.
    // SAFETY: The string is statically allocated (for `'static`).
    // PANIC: The string contains only ASCII characters.
    unsafe { CStr::from_ptr(name) }.to_str().unwrap()
}

macro_rules! enum_variants {
    ($name:ident, $inner:ident, [$( $value:ident ),* $(,)?] $(,)?) => {
        impl $name {
            $(
                /// Enum variant
                #[doc = paste::paste! { concat!("[`", stringify!([<$inner:upper _ $value>]), "`](open62541_sys::", stringify!([<$inner:upper _ $value>]), ")") }]
                /// from [`open62541_sys`].
                #[allow(dead_code)] // Allow unused `pub`-declared constants in private modules.
                pub const $value: Self = Self(
                    paste::paste! { open62541_sys::[<$inner:upper _ $value>] }
                );

                paste::paste! {
                    // This cast is necessary on Windows builds with inner type `i32`.
                    #[allow(clippy::as_conversions, trivial_numeric_casts)]
                    pub const [<$value _U32>]: u32 = open62541_sys::[<$inner:upper _ $value>];
                }
            )*
        }
    };
}

enum_variants!(
    StatusCode,
    UA_StatusCode,
    [
        GOOD,
        UNCERTAIN,
        BAD,
        BADUNEXPECTEDERROR,
        BADINTERNALERROR,
        BADOUTOFMEMORY,
        BADRESOURCEUNAVAILABLE,
        BADCOMMUNICATIONERROR,
        BADENCODINGERROR,
        BADDECODINGERROR,
        BADENCODINGLIMITSEXCEEDED,
        BADREQUESTTOOLARGE,
        BADRESPONSETOOLARGE,
        BADUNKNOWNRESPONSE,
        BADTIMEOUT,
        BADSERVICEUNSUPPORTED,
        BADSHUTDOWN,
        BADSERVERNOTCONNECTED,
        BADSERVERHALTED,
        BADNOTHINGTODO,
        BADTOOMANYOPERATIONS,
        BADTOOMANYMONITOREDITEMS,
        BADDATATYPEIDUNKNOWN,
        BADCERTIFICATEINVALID,
        BADSECURITYCHECKSFAILED,
        BADCERTIFICATEPOLICYCHECKFAILED,
        BADCERTIFICATETIMEINVALID,
        BADCERTIFICATEISSUERTIMEINVALID,
        BADCERTIFICATEHOSTNAMEINVALID,
        BADCERTIFICATEURIINVALID,
        BADCERTIFICATEUSENOTALLOWED,
        BADCERTIFICATEISSUERUSENOTALLOWED,
        BADCERTIFICATEUNTRUSTED,
        BADCERTIFICATEREVOCATIONUNKNOWN,
        BADCERTIFICATEISSUERREVOCATIONUNKNOWN,
        BADCERTIFICATEREVOKED,
        BADCERTIFICATEISSUERREVOKED,
        BADCERTIFICATECHAININCOMPLETE,
        BADUSERACCESSDENIED,
        BADIDENTITYTOKENINVALID,
        BADIDENTITYTOKENREJECTED,
        BADSECURECHANNELIDINVALID,
        BADINVALIDTIMESTAMP,
        BADNONCEINVALID,
        BADSESSIONIDINVALID,
        BADSESSIONCLOSED,
        BADSESSIONNOTACTIVATED,
        BADSUBSCRIPTIONIDINVALID,
        BADREQUESTHEADERINVALID,
        BADTIMESTAMPSTORETURNINVALID,
        BADREQUESTCANCELLEDBYCLIENT,
        BADTOOMANYARGUMENTS,
        BADLICENSEEXPIRED,
        BADLICENSELIMITSEXCEEDED,
        BADLICENSENOTAVAILABLE,
        GOODSUBSCRIPTIONTRANSFERRED,
        GOODCOMPLETESASYNCHRONOUSLY,
        GOODOVERLOAD,
        GOODCLAMPED,
        BADNOCOMMUNICATION,
        BADWAITINGFORINITIALDATA,
        BADNODEIDINVALID,
        BADNODEIDUNKNOWN,
        BADATTRIBUTEIDINVALID,
        BADINDEXRANGEINVALID,
        BADINDEXRANGENODATA,
        BADDATAENCODINGINVALID,
        BADDATAENCODINGUNSUPPORTED,
        BADNOTREADABLE,
        BADNOTWRITABLE,
        BADOUTOFRANGE,
        BADNOTSUPPORTED,
        BADNOTFOUND,
        BADOBJECTDELETED,
        BADNOTIMPLEMENTED,
        BADMONITORINGMODEINVALID,
        BADMONITOREDITEMIDINVALID,
        BADMONITOREDITEMFILTERINVALID,
        BADMONITOREDITEMFILTERUNSUPPORTED,
        BADFILTERNOTALLOWED,
        BADSTRUCTUREMISSING,
        BADEVENTFILTERINVALID,
        BADCONTENTFILTERINVALID,
        BADFILTEROPERATORINVALID,
        BADFILTEROPERATORUNSUPPORTED,
        BADFILTEROPERANDCOUNTMISMATCH,
        BADFILTEROPERANDINVALID,
        BADFILTERELEMENTINVALID,
        BADFILTERLITERALINVALID,
        BADCONTINUATIONPOINTINVALID,
        BADNOCONTINUATIONPOINTS,
        BADREFERENCETYPEIDINVALID,
        BADBROWSEDIRECTIONINVALID,
        BADNODENOTINVIEW,
        BADNUMERICOVERFLOW,
        BADSERVERURIINVALID,
        BADSERVERNAMEMISSING,
        BADDISCOVERYURLMISSING,
        BADSEMPAHOREFILEMISSING,
        BADREQUESTTYPEINVALID,
        BADSECURITYMODEREJECTED,
        BADSECURITYPOLICYREJECTED,
        BADTOOMANYSESSIONS,
        BADUSERSIGNATUREINVALID,
        BADAPPLICATIONSIGNATUREINVALID,
        BADNOVALIDCERTIFICATES,
        BADIDENTITYCHANGENOTSUPPORTED,
        BADREQUESTCANCELLEDBYREQUEST,
        BADPARENTNODEIDINVALID,
        BADREFERENCENOTALLOWED,
        BADNODEIDREJECTED,
        BADNODEIDEXISTS,
        BADNODECLASSINVALID,
        BADBROWSENAMEINVALID,
        BADBROWSENAMEDUPLICATED,
        BADNODEATTRIBUTESINVALID,
        BADTYPEDEFINITIONINVALID,
        BADSOURCENODEIDINVALID,
        BADTARGETNODEIDINVALID,
        BADDUPLICATEREFERENCENOTALLOWED,
        BADINVALIDSELFREFERENCE,
        BADREFERENCELOCALONLY,
        BADNODELETERIGHTS,
        UNCERTAINREFERENCENOTDELETED,
        BADSERVERINDEXINVALID,
        BADVIEWIDUNKNOWN,
        BADVIEWTIMESTAMPINVALID,
        BADVIEWPARAMETERMISMATCH,
        BADVIEWVERSIONINVALID,
        UNCERTAINNOTALLNODESAVAILABLE,
        GOODRESULTSMAYBEINCOMPLETE,
        BADNOTTYPEDEFINITION,
        UNCERTAINREFERENCEOUTOFSERVER,
        BADTOOMANYMATCHES,
        BADQUERYTOOCOMPLEX,
        BADNOMATCH,
        BADMAXAGEINVALID,
        BADSECURITYMODEINSUFFICIENT,
        BADHISTORYOPERATIONINVALID,
        BADHISTORYOPERATIONUNSUPPORTED,
        BADINVALIDTIMESTAMPARGUMENT,
        BADWRITENOTSUPPORTED,
        BADTYPEMISMATCH,
        BADMETHODINVALID,
        BADARGUMENTSMISSING,
        BADNOTEXECUTABLE,
        BADTOOMANYSUBSCRIPTIONS,
        BADTOOMANYPUBLISHREQUESTS,
        BADNOSUBSCRIPTION,
        BADSEQUENCENUMBERUNKNOWN,
        GOODRETRANSMISSIONQUEUENOTSUPPORTED,
        BADMESSAGENOTAVAILABLE,
        BADINSUFFICIENTCLIENTPROFILE,
        BADSTATENOTACTIVE,
        BADALREADYEXISTS,
        BADTCPSERVERTOOBUSY,
        BADTCPMESSAGETYPEINVALID,
        BADTCPSECURECHANNELUNKNOWN,
        BADTCPMESSAGETOOLARGE,
        BADTCPNOTENOUGHRESOURCES,
        BADTCPINTERNALERROR,
        BADTCPENDPOINTURLINVALID,
        BADREQUESTINTERRUPTED,
        BADREQUESTTIMEOUT,
        BADSECURECHANNELCLOSED,
        BADSECURECHANNELTOKENUNKNOWN,
        BADSEQUENCENUMBERINVALID,
        BADPROTOCOLVERSIONUNSUPPORTED,
        BADCONFIGURATIONERROR,
        BADNOTCONNECTED,
        BADDEVICEFAILURE,
        BADSENSORFAILURE,
        BADOUTOFSERVICE,
        BADDEADBANDFILTERINVALID,
        UNCERTAINNOCOMMUNICATIONLASTUSABLEVALUE,
        UNCERTAINLASTUSABLEVALUE,
        UNCERTAINSUBSTITUTEVALUE,
        UNCERTAININITIALVALUE,
        UNCERTAINSENSORNOTACCURATE,
        UNCERTAINENGINEERINGUNITSEXCEEDED,
        UNCERTAINSUBNORMAL,
        GOODLOCALOVERRIDE,
        BADREFRESHINPROGRESS,
        BADCONDITIONALREADYDISABLED,
        BADCONDITIONALREADYENABLED,
        BADCONDITIONDISABLED,
        BADEVENTIDUNKNOWN,
        BADEVENTNOTACKNOWLEDGEABLE,
        BADDIALOGNOTACTIVE,
        BADDIALOGRESPONSEINVALID,
        BADCONDITIONBRANCHALREADYACKED,
        BADCONDITIONBRANCHALREADYCONFIRMED,
        BADCONDITIONALREADYSHELVED,
        BADCONDITIONNOTSHELVED,
        BADSHELVINGTIMEOUTOFRANGE,
        BADNODATA,
        BADBOUNDNOTFOUND,
        BADBOUNDNOTSUPPORTED,
        BADDATALOST,
        BADDATAUNAVAILABLE,
        BADENTRYEXISTS,
        BADNOENTRYEXISTS,
        BADTIMESTAMPNOTSUPPORTED,
        GOODENTRYINSERTED,
        GOODENTRYREPLACED,
        UNCERTAINDATASUBNORMAL,
        GOODNODATA,
        GOODMOREDATA,
        BADAGGREGATELISTMISMATCH,
        BADAGGREGATENOTSUPPORTED,
        BADAGGREGATEINVALIDINPUTS,
        BADAGGREGATECONFIGURATIONREJECTED,
        GOODDATAIGNORED,
        BADREQUESTNOTALLOWED,
        BADREQUESTNOTCOMPLETE,
        BADTICKETREQUIRED,
        BADTICKETINVALID,
        GOODEDITED,
        GOODPOSTACTIONFAILED,
        UNCERTAINDOMINANTVALUECHANGED,
        GOODDEPENDENTVALUECHANGED,
        BADDOMINANTVALUECHANGED,
        UNCERTAINDEPENDENTVALUECHANGED,
        BADDEPENDENTVALUECHANGED,
        GOODEDITED_DEPENDENTVALUECHANGED,
        GOODEDITED_DOMINANTVALUECHANGED,
        GOODEDITED_DOMINANTVALUECHANGED_DEPENDENTVALUECHANGED,
        BADEDITED_OUTOFRANGE,
        BADINITIALVALUE_OUTOFRANGE,
        BADOUTOFRANGE_DOMINANTVALUECHANGED,
        BADEDITED_OUTOFRANGE_DOMINANTVALUECHANGED,
        BADOUTOFRANGE_DOMINANTVALUECHANGED_DEPENDENTVALUECHANGED,
        BADEDITED_OUTOFRANGE_DOMINANTVALUECHANGED_DEPENDENTVALUECHANGED,
        GOODCOMMUNICATIONEVENT,
        GOODSHUTDOWNEVENT,
        GOODCALLAGAIN,
        GOODNONCRITICALTIMEOUT,
        BADINVALIDARGUMENT,
        BADCONNECTIONREJECTED,
        BADDISCONNECT,
        BADCONNECTIONCLOSED,
        BADINVALIDSTATE,
        BADENDOFSTREAM,
        BADNODATAAVAILABLE,
        BADWAITINGFORRESPONSE,
        BADOPERATIONABANDONED,
        BADEXPECTEDSTREAMTOBLOCK,
        BADWOULDBLOCK,
        BADSYNTAXERROR,
        BADMAXCONNECTIONSREACHED,
    ],
);
