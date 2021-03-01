//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Session {
namespace Results {

/// <summary>
/// Enumeration for the reason the Session stopped.
/// </summary>
enum class SessionStoppedReason
{
    /// <summary>
    /// An error occurred.
    /// </summary>
    Error = -1,

    /// <summary>
    /// The end of the input stream was reached.
    /// </summary>
    NoMoreData = 0,

    /// <summary>
    /// An API call was made to stop recognizing.
    /// </summary>
    StopRequested = 1
};

} } } } } } // Azure::AI::Vision::Core::Session::Results